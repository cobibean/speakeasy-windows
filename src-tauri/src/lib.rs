mod pipeline;
mod settings;

use std::sync::Mutex;

use pipeline::{PipelineResult, PipelineSettings};
use settings::AppSettings;
use tauri::Manager;

struct AppState {
    settings: Mutex<AppSettings>,
    recorder: Mutex<RecorderState>,
}

#[derive(Debug, Default)]
struct RecorderState {
    is_recording: bool,
}

impl RecorderState {
    fn start(&mut self) {
        self.is_recording = true;
    }

    fn stop(&mut self) -> Result<(), String> {
        if !self.is_recording {
            return Err("Recording has not started".to_string());
        }

        self.is_recording = false;
        Ok(())
    }
}

#[tauri::command]
fn get_settings(state: tauri::State<'_, AppState>) -> Result<AppSettings, String> {
    state
        .settings
        .lock()
        .map(|settings| settings.clone())
        .map_err(|_| "Settings state is unavailable".to_string())
}

#[tauri::command]
fn save_settings(
    settings: AppSettings,
    state: tauri::State<'_, AppState>,
) -> Result<AppSettings, String> {
    let sanitized = settings.sanitized();
    let mut current = state
        .settings
        .lock()
        .map_err(|_| "Settings state is unavailable".to_string())?;
    *current = sanitized.clone();
    Ok(sanitized)
}

#[tauri::command]
fn start_hold_to_talk(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut recorder = state
        .recorder
        .lock()
        .map_err(|_| "Recorder state is unavailable".to_string())?;
    recorder.start();

    // Windows placeholder:
    // Replace this with global hotkey registration and WASAPI microphone capture after
    // human testing on Windows hardware. The command intentionally records no audio.
    Ok(())
}

#[tauri::command]
fn stop_and_run_pipeline(state: tauri::State<'_, AppState>) -> Result<PipelineResult, String> {
    {
        let mut recorder = state
            .recorder
            .lock()
            .map_err(|_| "Recorder state is unavailable".to_string())?;
        recorder.stop()?;
    }

    let settings = state
        .settings
        .lock()
        .map(|settings| settings.clone())
        .map_err(|_| "Settings state is unavailable".to_string())?;

    // Windows placeholder:
    // Future integration points are microphone bytes -> STT provider -> optional cleanup
    // -> active-window paste via Windows APIs. Until those are tested on Windows, this
    // returns deterministic local text and never contacts a provider.
    Ok(pipeline::run_placeholder_pipeline(PipelineSettings {
        cleanup_enabled: settings.cleanup_enabled,
        cleanup_strength: settings.cleanup_strength,
        paste_after_transcription: settings.paste_after_transcription,
    }))
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            settings: Mutex::new(AppSettings::default()),
            recorder: Mutex::new(RecorderState::default()),
        })
        .setup(|app| {
            let _main_window = app.get_webview_window("main");
            // Windows placeholders still needed before human testing:
            // tray menu lifecycle, single-instance behavior, installer metadata,
            // code signing, auto-start choice, and active-window focus restoration.
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            start_hold_to_talk,
            stop_and_run_pipeline
        ])
        .run(tauri::generate_context!())
        .expect("error while running Speakeasy Windows");
}

#[cfg(test)]
mod tests {
    use super::RecorderState;

    #[test]
    fn recorder_rejects_stop_before_start() {
        let mut recorder = RecorderState::default();

        assert_eq!(
            recorder.stop(),
            Err("Recording has not started".to_string())
        );
    }

    #[test]
    fn recorder_stops_after_start() {
        let mut recorder = RecorderState::default();

        recorder.start();
        assert!(recorder.stop().is_ok());
        assert!(!recorder.is_recording);
    }
}
