use serde::{Deserialize, Serialize};

use crate::settings::CleanupStrength;

const GROQ_TRANSCRIPTIONS_URL: &str = "https://api.groq.com/openai/v1/audio/transcriptions";
const GROQ_CHAT_URL: &str = "https://api.groq.com/openai/v1/chat/completions";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineSettings {
    pub groq_api_key: String,
    pub stt_model: String,
    pub cleanup_enabled: bool,
    pub cleanup_strength: CleanupStrength,
    pub cleanup_model: String,
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

#[derive(Debug, Deserialize)]
struct TranscriptionResponse {
    text: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatMessage {
    content: String,
}

pub async fn run_groq_pipeline(
    settings: PipelineSettings,
    audio_bytes: Vec<u8>,
    mime_type: String,
) -> Result<PipelineResult, String> {
    if settings.groq_api_key.trim().is_empty() {
        return Err("Enter a Groq API key before recording.".to_string());
    }

    if audio_bytes.is_empty() {
        return Err("No audio was captured. Check microphone access and try again.".to_string());
    }

    let client = reqwest::Client::new();
    let transcript = transcribe_audio(&client, &settings, audio_bytes, &mime_type).await?;
    let mut completed_stages = vec![PipelineStage::Transcribing];

    let polished_text = if settings.cleanup_enabled {
        completed_stages.push(PipelineStage::Polishing);
        cleanup_transcript(&client, &settings, &transcript).await?
    } else {
        transcript.clone()
    };

    let pasted = false;
    if pasted {
        completed_stages.push(PipelineStage::Pasting);
    }

    Ok(PipelineResult {
        transcript,
        polished_text,
        pasted,
        used_cleanup: settings.cleanup_enabled,
        placeholder: false,
        completed_stages,
    })
}

async fn transcribe_audio(
    client: &reqwest::Client,
    settings: &PipelineSettings,
    audio_bytes: Vec<u8>,
    mime_type: &str,
) -> Result<String, String> {
    let file_name = format!("speakeasy-recording.{}", extension_for_mime(mime_type));
    let file_part = reqwest::multipart::Part::bytes(audio_bytes)
        .file_name(file_name)
        .mime_str(mime_type)
        .map_err(|_| "Captured audio format is not supported.".to_string())?;
    let form = reqwest::multipart::Form::new()
        .part("file", file_part)
        .text("model", settings.stt_model.clone())
        .text("response_format", "json")
        .text("temperature", "0");

    let response = client
        .post(GROQ_TRANSCRIPTIONS_URL)
        .bearer_auth(&settings.groq_api_key)
        .multipart(form)
        .send()
        .await
        .map_err(|error| format!("Groq transcription request failed: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Groq transcription failed ({status}): {body}"));
    }

    let payload: TranscriptionResponse = response
        .json()
        .await
        .map_err(|error| format!("Groq transcription response was invalid: {error}"))?;
    let text = payload.text.trim().to_string();

    if text.is_empty() {
        return Err("Groq returned an empty transcript.".to_string());
    }

    Ok(text)
}

async fn cleanup_transcript(
    client: &reqwest::Client,
    settings: &PipelineSettings,
    transcript: &str,
) -> Result<String, String> {
    let instruction = cleanup_instruction(settings.cleanup_strength);
    let body = serde_json::json!({
        "model": settings.cleanup_model,
        "temperature": 0.1,
        "messages": [
            {
                "role": "system",
                "content": instruction
            },
            {
                "role": "user",
                "content": transcript
            }
        ]
    });

    let response = client
        .post(GROQ_CHAT_URL)
        .bearer_auth(&settings.groq_api_key)
        .json(&body)
        .send()
        .await
        .map_err(|error| format!("Groq cleanup request failed: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Groq cleanup failed ({status}): {body}"));
    }

    let payload: ChatCompletionResponse = response
        .json()
        .await
        .map_err(|error| format!("Groq cleanup response was invalid: {error}"))?;

    payload
        .choices
        .first()
        .map(|choice| choice.message.content.trim().to_string())
        .filter(|content| !content.is_empty())
        .ok_or_else(|| "Groq cleanup returned no text.".to_string())
}

fn cleanup_instruction(strength: CleanupStrength) -> &'static str {
    match strength {
        CleanupStrength::Light => {
            "Clean up obvious speech-to-text errors while preserving the speaker's wording. Return only the cleaned text."
        }
        CleanupStrength::Standard => {
            "Clean this dictated text for punctuation, capitalization, and small grammar fixes. Preserve meaning and voice. Return only the cleaned text."
        }
        CleanupStrength::Strong => {
            "Rewrite this dictated text into clear polished prose while preserving meaning. Return only the final text."
        }
    }
}

fn extension_for_mime(mime_type: &str) -> &'static str {
    if mime_type.contains("mp4") {
        "mp4"
    } else if mime_type.contains("mpeg") || mime_type.contains("mp3") {
        "mp3"
    } else if mime_type.contains("ogg") {
        "ogg"
    } else if mime_type.contains("wav") {
        "wav"
    } else {
        "webm"
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn extension_for_mime_prefers_supported_groq_audio_extensions() {
        assert_eq!(super::extension_for_mime("audio/webm;codecs=opus"), "webm");
        assert_eq!(super::extension_for_mime("audio/mp4"), "mp4");
        assert_eq!(super::extension_for_mime("audio/wav"), "wav");
    }
}
