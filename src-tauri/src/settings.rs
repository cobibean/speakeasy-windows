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
    pub cleanup_enabled: bool,
    pub cleanup_strength: CleanupStrength,
    pub paste_after_transcription: bool,
    pub provider_mode: ProviderMode,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ProviderMode {
    Mock,
    Manual,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            hold_to_talk_hotkey: "Ctrl+Alt+Space".to_string(),
            cleanup_enabled: true,
            cleanup_strength: CleanupStrength::Standard,
            paste_after_transcription: true,
            provider_mode: ProviderMode::Mock,
        }
    }
}

impl AppSettings {
    pub fn sanitized(self) -> Self {
        Self {
            hold_to_talk_hotkey: sanitize_hotkey(&self.hold_to_talk_hotkey),
            cleanup_enabled: self.cleanup_enabled,
            cleanup_strength: self.cleanup_strength,
            paste_after_transcription: self.paste_after_transcription,
            provider_mode: self.provider_mode,
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

#[cfg(test)]
mod tests {
    use super::{sanitize_hotkey, AppSettings};

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
}
