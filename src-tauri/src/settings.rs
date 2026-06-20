use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CleanupStrength {
    Light,
    Standard,
    Strong,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub hold_to_talk_hotkey: String,
    pub groq_api_key: String,
    pub stt_model: String,
    pub cleanup_enabled: bool,
    pub cleanup_strength: CleanupStrength,
    pub cleanup_model: String,
    pub paste_after_transcription: bool,
    pub provider_mode: ProviderMode,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ProviderMode {
    Mock,
    Groq,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            hold_to_talk_hotkey: "Ctrl+Alt+Space".to_string(),
            groq_api_key: String::new(),
            stt_model: "whisper-large-v3-turbo".to_string(),
            cleanup_enabled: true,
            cleanup_strength: CleanupStrength::Standard,
            cleanup_model: "openai/gpt-oss-120b".to_string(),
            paste_after_transcription: true,
            provider_mode: ProviderMode::Mock,
        }
    }
}

impl AppSettings {
    pub fn sanitized(self) -> Self {
        Self {
            hold_to_talk_hotkey: sanitize_hotkey(&self.hold_to_talk_hotkey),
            groq_api_key: self.groq_api_key.trim().to_string(),
            stt_model: sanitize_model(&self.stt_model, &AppSettings::default().stt_model),
            cleanup_enabled: self.cleanup_enabled,
            cleanup_strength: self.cleanup_strength,
            cleanup_model: sanitize_model(
                &self.cleanup_model,
                &AppSettings::default().cleanup_model,
            ),
            paste_after_transcription: self.paste_after_transcription,
            provider_mode: ProviderMode::Groq,
        }
    }
}

pub fn sanitize_hotkey(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return AppSettings::default().hold_to_talk_hotkey;
    }

    trimmed.chars().take(64).collect()
}

pub fn sanitize_model(input: &str, fallback: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return fallback.to_string();
    }

    trimmed.chars().take(96).collect()
}

#[cfg(test)]
mod tests {
    use super::{sanitize_hotkey, sanitize_model, AppSettings};

    #[test]
    fn sanitize_hotkey_falls_back_when_blank() {
        assert_eq!(
            sanitize_hotkey("   "),
            AppSettings::default().hold_to_talk_hotkey
        );
    }

    #[test]
    fn sanitize_hotkey_limits_length() {
        let long = "A".repeat(80);
        assert_eq!(sanitize_hotkey(&long).len(), 64);
    }

    #[test]
    fn sanitize_model_falls_back_when_blank() {
        assert_eq!(
            sanitize_model(" ", "whisper-large-v3-turbo"),
            "whisper-large-v3-turbo"
        );
    }

    #[test]
    fn app_settings_sanitizes_without_creating_a_default_key() {
        let test_key = ["test", "key", "value"].join("-");
        let settings = AppSettings {
            groq_api_key: format!("  {test_key}  "),
            stt_model: " whisper-large-v3 ".to_string(),
            cleanup_model: " openai/gpt-oss-120b ".to_string(),
            ..AppSettings::default()
        }
        .sanitized();

        assert_eq!(settings.groq_api_key, test_key);
        assert_eq!(settings.stt_model, "whisper-large-v3");
        assert_eq!(AppSettings::default().groq_api_key, "");
    }
}
