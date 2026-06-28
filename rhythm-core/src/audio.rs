use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use rodio::Source;

use crate::models::Pattern;

#[derive(Debug, Clone)]
pub enum PlaybackCommand {
    Play,
    Stop,
    Pause,
    SetBpm(u32),
    LoadPattern(Pattern),
}

#[derive(Debug, Clone)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_step: usize,
    pub bpm: u32,
}

pub struct AudioEngine {
    command_tx: Sender<PlaybackCommand>,
    state: Arc<Mutex<PlaybackState>>,
}

impl AudioEngine {
    pub fn new(samples_dir: Option<PathBuf>) -> Result<Self, String> {
        let mut samples: HashMap<String, Vec<u8>> = HashMap::new();
        if let Some(dir) = samples_dir {
            if dir.exists() {
                for entry in std::fs::read_dir(&dir).map_err(|e| format!("Failed to read samples dir: {}", e))? {
                    let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("wav") {
                        if let Ok(data) = std::fs::read(&path) {
                            let name = path
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .map(|s| s.to_string())
                                .unwrap_or_default();
                            samples.insert(name, data);
                        }
                    }
                }
            }
        }

        let (command_tx, command_rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(PlaybackState {
            is_playing: false,
            current_step: 0,
            bpm: 120,
        }));
        let state_clone = state.clone();

        thread::spawn(move || {
            let (stream, stream_handle) = match rodio::OutputStream::try_default() {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("AudioEngine: failed to open audio output: {}", e);
                    loop {
                        while let Ok(cmd) = command_rx.try_recv() {
                            if let Ok(mut s) = state_clone.lock() {
                                match cmd {
                                    PlaybackCommand::Play => s.is_playing = true,
                                    PlaybackCommand::Stop | PlaybackCommand::Pause => {
                                        s.is_playing = false;
                                        s.current_step = 0;
                                    }
                                    PlaybackCommand::SetBpm(bpm) => s.bpm = bpm.clamp(20, 300),
                                    PlaybackCommand::LoadPattern(_) => {}
                                }
                            }
                        }
                        thread::sleep(std::time::Duration::from_millis(50));
                    }
                }
            };
            Box::leak(Box::new(stream));

            let mut current_pattern: Option<Pattern> = None;
            let mut current_step: usize = 0;
            let mut bpm: u32 = 120;
            let mut playing = false;
            let mut step_timer = std::time::Instant::now();

            loop {
                while let Ok(cmd) = command_rx.try_recv() {
                    match cmd {
                        PlaybackCommand::Play => {
                            if current_pattern.is_some() {
                                playing = true;
                                current_step = 0;
                                step_timer = std::time::Instant::now();
                                if let Ok(mut s) = state_clone.lock() {
                                    s.is_playing = true;
                                    s.current_step = 0;
                                }
                            }
                        }
                        PlaybackCommand::Stop => {
                            playing = false;
                            current_step = 0;
                            if let Ok(mut s) = state_clone.lock() {
                                s.is_playing = false;
                                s.current_step = 0;
                            }
                        }
                        PlaybackCommand::Pause => {
                            playing = false;
                            if let Ok(mut s) = state_clone.lock() {
                                s.is_playing = false;
                            }
                        }
                        PlaybackCommand::SetBpm(new_bpm) => {
                            bpm = new_bpm.clamp(20, 300);
                            if let Ok(mut s) = state_clone.lock() {
                                s.bpm = bpm;
                            }
                        }
                        PlaybackCommand::LoadPattern(pattern) => {
                            current_pattern = Some(pattern);
                        }
                    }
                }

                if playing {
                    if let Some(ref pattern) = current_pattern {
                        let beats_per_second = bpm as f64 / 60.0;
                        let steps_per_beat = 4.0;
                        let step_duration =
                            std::time::Duration::from_secs_f64(1.0 / (beats_per_second * steps_per_beat));

                        if step_timer.elapsed() >= step_duration {
                            for inst in &pattern.instruments {
                                if let Some(steps) = pattern.grid.get(&inst.id) {
                                    if steps.get(current_step).copied().unwrap_or(false) {
                                        if let Some(ref sample_name) = inst.sample {
                                            if let Some(sample_data) = samples.get(sample_name) {
                                                let cursor = Cursor::new(sample_data.clone());
                                                if let Ok(source) = rodio::Decoder::new_wav(cursor) {
                                                    if let Ok(sink) =
                                                        rodio::Sink::try_new(&stream_handle)
                                                    {
                                                        let vol = inst.volume;
                                                        sink.append(source.amplify(vol));
                                                        sink.detach();
                                                    }
                                                }
                                                continue;
                                            }
                                        }

                                        let freq = match inst.id.as_str() {
                                            "kick" => 80.0,
                                            "snare" => 200.0,
                                            "hihat" => 800.0,
                                            _ => 440.0,
                                        };
                                        let source = rodio::source::SineWave::new(freq)
                                            .take_duration(std::time::Duration::from_millis(80))
                                            .amplify(inst.volume);
                                        if let Ok(sink) = rodio::Sink::try_new(&stream_handle) {
                                            sink.append(source);
                                            sink.detach();
                                        }
                                    }
                                }
                            }

                            current_step = (current_step + 1) % pattern.steps as usize;
                            step_timer = std::time::Instant::now();

                            if let Ok(mut s) = state_clone.lock() {
                                s.current_step = current_step;
                            }
                        }
                    }
                }

                thread::sleep(std::time::Duration::from_millis(5));
            }
        });

        Ok(Self { command_tx, state })
    }

    pub fn load_pattern(&self, pattern: Pattern) {
        let _ = self.command_tx.send(PlaybackCommand::LoadPattern(pattern));
    }

    pub fn play(&self) {
        let _ = self.command_tx.send(PlaybackCommand::Play);
    }

    pub fn stop(&self) {
        let _ = self.command_tx.send(PlaybackCommand::Stop);
    }

    pub fn pause(&self) {
        let _ = self.command_tx.send(PlaybackCommand::Pause);
    }

    pub fn set_bpm(&self, bpm: u32) {
        let _ = self.command_tx.send(PlaybackCommand::SetBpm(bpm));
    }

    pub fn get_state(&self) -> PlaybackState {
        self.state
            .lock()
            .map(|s| s.clone())
            .unwrap_or(PlaybackState {
                is_playing: false,
                current_step: 0,
                bpm: 120,
            })
    }
}
