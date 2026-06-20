use serde::{Deserialize, Serialize};

use crate::settings::CleanupStrength;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineSettings {
    pub cleanup_enabled: bool,
    pub cleanup_strength: CleanupStrength,
    pub paste_after_transcription: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PipelineResult {
    pub transcript: String,
    pub polished_text: String,
    pub pasted: bool,
    pub used_cleanup: bool,
    pub placeholder: bool,
    pub completed_stages: Vec<PipelineStage>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PipelineStage {
    Transcribing,
    Polishing,
    Pasting,
}

pub fn run_placeholder_pipeline(settings: PipelineSettings) -> PipelineResult {
    let transcript = "mock transcript from the windows prototype loop";
    let mut completed_stages = vec![PipelineStage::Transcribing];
    let polished_text = if settings.cleanup_enabled {
        completed_stages.push(PipelineStage::Polishing);
        polish_placeholder(transcript, settings.cleanup_strength)
    } else {
        transcript.to_string()
    };

    let pasted = false;
    if pasted {
        completed_stages.push(PipelineStage::Pasting);
    }

    PipelineResult {
        transcript: transcript.to_string(),
        polished_text,
        pasted,
        used_cleanup: settings.cleanup_enabled,
        placeholder: true,
        completed_stages,
    }
}

pub fn polish_placeholder(input: &str, strength: CleanupStrength) -> String {
    let normalized = input.trim();
    if normalized.is_empty() {
        return String::new();
    }

    let mut chars = normalized.chars();
    let mut sentence = match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    };

    match strength {
        CleanupStrength::Light => sentence,
        CleanupStrength::Standard => {
            if !sentence.ends_with('.') {
                sentence.push('.');
            }
            sentence
        }
        CleanupStrength::Strong => {
            if !sentence.ends_with('.') {
                sentence.push('.');
            }
            format!("{sentence} [cleanup placeholder]")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{polish_placeholder, run_placeholder_pipeline, PipelineSettings};
    use crate::settings::CleanupStrength;

    #[test]
    fn polish_placeholder_capitalizes_and_punctuates_standard_text() {
        assert_eq!(
            polish_placeholder("  hello windows  ", CleanupStrength::Standard),
            "Hello windows."
        );
    }

    #[test]
    fn pipeline_does_not_claim_real_paste_in_placeholder_mode() {
        let result = run_placeholder_pipeline(PipelineSettings {
            cleanup_enabled: true,
            cleanup_strength: CleanupStrength::Standard,
            paste_after_transcription: true,
        });

        assert!(result.placeholder);
        assert!(!result.pasted);
        assert!(result.used_cleanup);
        assert_eq!(
            result.completed_stages,
            vec![
                crate::pipeline::PipelineStage::Transcribing,
                crate::pipeline::PipelineStage::Polishing
            ]
        );
    }
}
