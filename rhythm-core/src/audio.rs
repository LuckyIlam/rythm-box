use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use rodio::Source;

use crate::error::AudioError;
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

fn load_samples(samples_dir: Option<PathBuf>) -> HashMap<String, Vec<u8>> {
    let mut samples = HashMap::new();
    if let Some(dir) = samples_dir {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
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
    }
    samples
}

fn playback_thread(
    command_rx: mpsc::Receiver<PlaybackCommand>,
    state_clone: Arc<Mutex<PlaybackState>>,
    mut current_pattern: Option<Pattern>,
    mut bpm: u32,
    samples: HashMap<String, Vec<u8>>,
    stream_handle: Option<rodio::OutputStreamHandle>,
) {
    let mut current_step: usize = 0;
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
                    if let Some(ref handle) = stream_handle {
                        for inst in &pattern.instruments {
                            if let Some(steps) = pattern.grid.get(&inst.id) {
                                if steps.get(current_step).copied().unwrap_or(false) {
                                    if let Some(ref sample_name) = inst.sample {
                                        if let Some(sample_data) = samples.get(sample_name) {
                                            let cursor = Cursor::new(sample_data.clone());
                                            if let Ok(source) = rodio::Decoder::new_wav(cursor) {
                                                if let Ok(sink) = rodio::Sink::try_new(handle) {
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
                                    if let Ok(sink) = rodio::Sink::try_new(handle) {
                                        sink.append(source);
                                        sink.detach();
                                    }
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
}

impl AudioEngine {
    pub fn new(samples_dir: Option<PathBuf>) -> Result<Self, AudioError> {
        let samples = load_samples(samples_dir);

        let (command_tx, command_rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(PlaybackState {
            is_playing: false,
            current_step: 0,
            bpm: 120,
        }));
        let state_clone = state.clone();

        let stream_handle = match rodio::OutputStream::try_default() {
            Ok((stream, handle)) => {
                Box::leak(Box::new(stream));
                Some(handle)
            }
            Err(e) => {
                eprintln!("AudioEngine: failed to open audio output: {}", e);
                None
            }
        };

        thread::spawn(move || {
            playback_thread(command_rx, state_clone, None, 120, samples, stream_handle);
        });

        Ok(Self { command_tx, state })
    }

    #[cfg(test)]
    pub(crate) fn new_test() -> Self {
        let samples = HashMap::new();
        let (command_tx, command_rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(PlaybackState {
            is_playing: false,
            current_step: 0,
            bpm: 120,
        }));
        let state_clone = state.clone();

        thread::spawn(move || {
            playback_thread(command_rx, state_clone, None, 120, samples, None);
        });

        Self { command_tx, state }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_initial_state() {
        let engine = AudioEngine::new_test();
        let state = engine.get_state();
        assert!(!state.is_playing);
        assert_eq!(state.current_step, 0);
        assert_eq!(state.bpm, 120);
    }

    #[test]
    fn test_play_command_updates_state() {
        let engine = AudioEngine::new_test();
        let pattern = Pattern::default();
        engine.load_pattern(pattern);
        engine.play();
        thread::sleep(Duration::from_millis(50));
        let state = engine.get_state();
        assert!(state.is_playing);
    }

    #[test]
    fn test_stop_command_updates_state() {
        let engine = AudioEngine::new_test();
        let pattern = Pattern::default();
        engine.load_pattern(pattern);
        engine.play();
        thread::sleep(Duration::from_millis(50));
        engine.stop();
        thread::sleep(Duration::from_millis(50));
        let state = engine.get_state();
        assert!(!state.is_playing);
        assert_eq!(state.current_step, 0);
    }

    #[test]
    fn test_pause_command_preserves_step() {
        let engine = AudioEngine::new_test();
        let pattern = Pattern::default();
        engine.load_pattern(pattern);
        engine.play();
        thread::sleep(Duration::from_millis(50));
        engine.pause();
        thread::sleep(Duration::from_millis(50));
        let state = engine.get_state();
        assert!(!state.is_playing);
    }

    #[test]
    fn test_set_bpm() {
        let engine = AudioEngine::new_test();
        engine.set_bpm(140);
        thread::sleep(Duration::from_millis(50));
        let state = engine.get_state();
        assert_eq!(state.bpm, 140);
    }

    #[test]
    fn test_set_bpm_clamps() {
        let engine = AudioEngine::new_test();
        engine.set_bpm(10);
        thread::sleep(Duration::from_millis(50));
        let state = engine.get_state();
        assert_eq!(state.bpm, 20);
    }

    #[test]
    fn test_set_bpm_clamps_high() {
        let engine = AudioEngine::new_test();
        engine.set_bpm(500);
        thread::sleep(Duration::from_millis(50));
        let state = engine.get_state();
        assert_eq!(state.bpm, 300);
    }

    #[test]
    fn test_play_without_pattern_is_noop() {
        let engine = AudioEngine::new_test();
        engine.play();
        thread::sleep(Duration::from_millis(50));
        let state = engine.get_state();
        assert!(!state.is_playing);
    }

    #[test]
    fn test_load_samples_directory_not_found() {
        let samples = load_samples(Some(PathBuf::from("/nonexistent/path")));
        assert!(samples.is_empty());
    }

    #[test]
    fn test_load_samples_none() {
        let samples = load_samples(None);
        assert!(samples.is_empty());
    }
}
