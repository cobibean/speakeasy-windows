mod pipeline;
mod settings;

use base64::{engine::general_purpose::STANDARD, Engine as _};
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

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AudioRequest {
    audio_base64: String,
    mime_type: String,
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
async fn stop_and_run_pipeline(
    request: AudioRequest,
    state: tauri::State<'_, AppState>,
) -> Result<PipelineResult, String> {
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
    let audio_bytes = STANDARD
        .decode(request.audio_base64)
        .map_err(|_| "Captured audio could not be decoded.".to_string())?;

    // Windows testing still needs to validate microphone permission prompts and active-window paste.
    // The provider path is real BYOK Groq, but paste remains clipboard-first until Windows APIs are wired.
    pipeline::run_groq_pipeline(
        PipelineSettings {
            groq_api_key: settings.groq_api_key,
            stt_model: settings.stt_model,
            cleanup_enabled: settings.cleanup_enabled,
            cleanup_strength: settings.cleanup_strength,
            cleanup_model: settings.cleanup_model,
            paste_after_transcription: settings.paste_after_transcription,
        },
        audio_bytes,
        request.mime_type,
    )
    .await
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
