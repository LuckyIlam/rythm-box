use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use rhythm_core::audio::AudioEngine;
use rhythm_core::models::Pattern;
use rhythm_core::project::Project;
use rhythm_core::validation::validate_project;
use tauri::Manager;

struct AppState {
    engine: Arc<Mutex<Option<AudioEngine>>>,
}

#[tauri::command]
fn save_project(path: String, project_json: String) -> Result<(), String> {
    let project: Project =
        Project::from_json(&project_json).map_err(|e| format!("Invalid project: {}", e))?;
    let errors = validate_project(&project);
    if !errors.is_empty() {
        return Err(format!(
            "Validation failed: {:?}",
            errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>()
        ));
    }
    std::fs::write(&path, &project_json).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}

#[tauri::command]
fn load_project(path: String) -> Result<String, String> {
    let content =
        std::fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;
    let project: Project =
        Project::from_json(&content).map_err(|e| format!("Invalid project file: {}", e))?;
    let errors = validate_project(&project);
    if !errors.is_empty() {
        return Err(format!(
            "Validation failed: {:?}",
            errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>()
        ));
    }
    Ok(content)
}

#[tauri::command]
fn get_project_json(project_json: String) -> Result<String, String> {
    let project: Project =
        Project::from_json(&project_json).map_err(|e| format!("Invalid project: {}", e))?;
    project
        .to_json()
        .map_err(|e| format!("Serialization failed: {}", e))
}

#[tauri::command]
fn play_pattern(state: tauri::State<AppState>, pattern_json: String) -> Result<(), String> {
    let pattern: Pattern =
        serde_json::from_str(&pattern_json).map_err(|e| format!("Invalid pattern: {}", e))?;
    if let Ok(guard) = state.engine.lock() {
        if let Some(ref engine) = *guard {
            engine.load_pattern(pattern);
            engine.play();
        }
    }
    Ok(())
}

#[tauri::command]
fn stop_audio(state: tauri::State<AppState>) {
    if let Ok(guard) = state.engine.lock() {
        if let Some(ref engine) = *guard {
            engine.stop();
        }
    }
}

#[tauri::command]
fn set_audio_bpm(state: tauri::State<AppState>, bpm: u32) {
    if let Ok(guard) = state.engine.lock() {
        if let Some(ref engine) = *guard {
            engine.set_bpm(bpm);
        }
    }
}

fn resolve_samples_dir(app: &tauri::App) -> PathBuf {
    let resource_dir = app
        .path()
        .resource_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    let samples_dir = resource_dir.join("samples");
    if samples_dir.exists() {
        return samples_dir;
    }
    let dev_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("samples");
    if dev_dir.exists() {
        return dev_dir;
    }
    samples_dir
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let samples_dir = resolve_samples_dir(app);
            let engine = AudioEngine::new(Some(samples_dir)).ok();
            app.manage(AppState {
                engine: Arc::new(Mutex::new(engine)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_project,
            load_project,
            get_project_json,
            play_pattern,
            stop_audio,
            set_audio_bpm,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
