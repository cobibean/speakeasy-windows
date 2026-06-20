import { useEffect, useMemo, useReducer, useRef, useState } from "react";
import { selectRecordingMimeType } from "../domain/audio";
import { createInitialSnapshot, appReducer, statusLabel } from "../domain/state";
import type { AppSettings, CapturedAudio, CleanupStrength, DictationStatus, PipelineStage } from "../domain/types";
import { loadSettings, saveSettings, startHoldToTalk, stopAndRunPipeline } from "../tauri/commands";

const cleanupOptions: CleanupStrength[] = ["light", "standard", "strong"];

export function App() {
  const [state, dispatch] = useReducer(appReducer, undefined, createInitialSnapshot);
  const [draftSettings, setDraftSettings] = useState<AppSettings>(state.settings);
  const recorderRef = useRef<MediaRecorder | null>(null);
  const streamRef = useRef<MediaStream | null>(null);
  const chunksRef = useRef<BlobPart[]>([]);
  const canStart =
    state.status === "idle" ||
    state.status === "complete" ||
    state.status === "error" ||
    state.status === "micAccessRequired";
  const canStop = state.status === "listening";

  useEffect(() => {
    loadSettings()
      .then((settings) => {
        dispatch({ type: "settingsLoaded", settings });
        setDraftSettings(settings);
      })
      .catch((error: unknown) => {
        dispatch({ type: "failed", message: toErrorMessage(error) });
      });
  }, []);

  const previewText = useMemo(() => {
    if (state.lastPolishedText) return state.lastPolishedText;
    if (state.lastTranscript) return state.lastTranscript;
    return "Enter a Groq API key, press Start, speak a short phrase, then Stop. The packaged app sends audio directly to Groq from your machine.";
  }, [state.lastPolishedText, state.lastTranscript]);

  async function handleStart() {
    try {
      const savedSettings = await saveSettings(draftSettings);
      dispatch({ type: "settingsSaved", settings: savedSettings });
      setDraftSettings(savedSettings);

      if (!savedSettings.groqApiKey.trim()) {
        dispatch({ type: "failed", message: "Enter your Groq API key before recording." });
        return;
      }

      if (!navigator.mediaDevices?.getUserMedia || !window.MediaRecorder) {
        throw new Error("Microphone recording is not available in this WebView.");
      }

      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const mimeType = selectRecordingMimeType(MediaRecorder);
      const recorder = new MediaRecorder(stream, mimeType ? { mimeType } : undefined);

      chunksRef.current = [];
      recorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          chunksRef.current.push(event.data);
        }
      };

      recorderRef.current = recorder;
      streamRef.current = stream;
      dispatch({ type: "statusChanged", status: "listening" });
      await startHoldToTalk();
      recorder.start();
    } catch (error) {
      dispatch({ type: "failed", status: "micAccessRequired", message: toErrorMessage(error) });
      stopMediaTracks();
    }
  }

  async function handleStop() {
    try {
      const capturedAudio = await stopRecording();
      dispatch({ type: "statusChanged", status: "transcribing" });
      const result = await stopAndRunPipeline(capturedAudio);
      for (const stage of result.completedStages) {
        const status = statusForStage(stage);
        if (status !== "transcribing") {
          await showVisibleStatus(status, dispatch);
        }
      }
      const pasted = await copyResultIfEnabled(result.polishedText || result.transcript, draftSettings.pasteAfterTranscription);
      if (pasted) {
        await showVisibleStatus("pasting", dispatch);
      }
      dispatch({ type: "pipelineCompleted", result: { ...result, pasted } });
    } catch (error) {
      dispatch({ type: "failed", message: toErrorMessage(error) });
    } finally {
      stopMediaTracks();
    }
  }

  async function handleSaveSettings() {
    try {
      const saved = await saveSettings(draftSettings);
      dispatch({ type: "settingsSaved", settings: saved });
      setDraftSettings(saved);
    } catch (error) {
      dispatch({ type: "failed", message: toErrorMessage(error) });
    }
  }

  function stopRecording(): Promise<CapturedAudio> {
    const recorder = recorderRef.current;
    if (!recorder) {
      return Promise.reject(new Error("Recorder is not running."));
    }

    const mimeType = recorder.mimeType || "audio/webm";

    return new Promise((resolve, reject) => {
      recorder.onstop = async () => {
        try {
          const blob = new Blob(chunksRef.current, { type: mimeType });
          resolve({ audioBase64: await blobToBase64(blob), mimeType });
        } catch (error) {
          reject(error);
        }
      };
      recorder.onerror = () => reject(new Error("Recording failed."));
      recorder.stop();
    });
  }

  function stopMediaTracks() {
    streamRef.current?.getTracks().forEach((track) => track.stop());
    streamRef.current = null;
    recorderRef.current = null;
    chunksRef.current = [];
  }

  return (
    <main className="app-shell">
      <section className="utility-window" aria-label="Speakeasy Windows utility">
        <header className="titlebar">
          <div>
            <h1>Speakeasy</h1>
            <p>Windows OSS prototype</p>
          </div>
          <div className={`status-light status-light--${state.status}`} aria-hidden="true" />
        </header>

        <section className="control-strip" aria-label="Dictation controls">
          <div>
            <span className="eyebrow">Status</span>
            <strong>{statusLabel(state.status)}</strong>
          </div>
          <div className="actions">
            <button type="button" className="primary-button" disabled={!canStart} onClick={handleStart}>
              Start
            </button>
            <button type="button" className="secondary-button" disabled={!canStop} onClick={handleStop}>
              Stop
            </button>
          </div>
        </section>

        {state.errorMessage ? (
          <div className="notice notice--error" role="alert">
            {state.errorMessage}
          </div>
        ) : (
          <div className="notice">
            Test build: your Groq key stays local to this running app and is sent only to Groq API requests.
          </div>
        )}

        <section className="preview-panel" aria-label="Transcript preview">
          <div className="panel-heading">
            <h2>Transcript Preview</h2>
            <span>{state.settings.cleanupEnabled ? "Cleanup on" : "Cleanup off"}</span>
          </div>
          <p>{previewText}</p>
        </section>

        <section className="settings-panel" aria-label="Settings">
          <div className="panel-heading">
            <h2>Settings</h2>
            <span>Local only</span>
          </div>

          <label className="field">
            <span>Hold-to-talk hotkey</span>
            <input
              value={draftSettings.holdToTalkHotkey}
              onChange={(event) => setDraftSettings({ ...draftSettings, holdToTalkHotkey: event.target.value })}
            />
          </label>

          <label className="field">
            <span>Groq API key</span>
            <input
              type="password"
              placeholder="gsk_..."
              value={draftSettings.groqApiKey}
              onChange={(event) => setDraftSettings({ ...draftSettings, groqApiKey: event.target.value })}
            />
          </label>

          <label className="field">
            <span>Speech model</span>
            <input
              value={draftSettings.sttModel}
              onChange={(event) => setDraftSettings({ ...draftSettings, sttModel: event.target.value })}
            />
          </label>

          <label className="toggle-row">
            <input
              type="checkbox"
              checked={draftSettings.cleanupEnabled}
              onChange={(event) => setDraftSettings({ ...draftSettings, cleanupEnabled: event.target.checked })}
            />
            <span>Clean up dictated text before paste</span>
          </label>

          <label className="field">
            <span>Cleanup strength</span>
            <select
              value={draftSettings.cleanupStrength}
              onChange={(event) =>
                setDraftSettings({ ...draftSettings, cleanupStrength: event.target.value as CleanupStrength })
              }
            >
              {cleanupOptions.map((option) => (
                <option key={option} value={option}>
                  {option}
                </option>
              ))}
            </select>
          </label>

          <label className="field">
            <span>Cleanup model</span>
            <input
              value={draftSettings.cleanupModel}
              onChange={(event) => setDraftSettings({ ...draftSettings, cleanupModel: event.target.value })}
            />
          </label>

          <label className="toggle-row">
            <input
              type="checkbox"
              checked={draftSettings.pasteAfterTranscription}
              onChange={(event) =>
                setDraftSettings({ ...draftSettings, pasteAfterTranscription: event.target.checked })
              }
            />
            <span>Copy result to clipboard after transcription</span>
          </label>

          <button type="button" className="secondary-button full-width" onClick={handleSaveSettings}>
            Save settings
          </button>
        </section>

        <footer className="platform-notes">
          Windows testing still needs to validate microphone permission prompts, clipboard paste behavior, tray behavior,
          installer, and signing.
        </footer>
      </section>
    </main>
  );
}

function blobToBase64(blob: Blob): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onerror = () => reject(new Error("Captured audio could not be read."));
    reader.onloadend = () => {
      const result = typeof reader.result === "string" ? reader.result : "";
      resolve(result.split(",")[1] ?? "");
    };
    reader.readAsDataURL(blob);
  });
}

async function copyResultIfEnabled(text: string, enabled: boolean): Promise<boolean> {
  if (!enabled || !text.trim()) return false;
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch {
    return false;
  }
}

function toErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message;
  if (typeof error === "string") return error;
  return "Unexpected prototype error";
}

function statusForStage(stage: PipelineStage): Extract<DictationStatus, "transcribing" | "polishing" | "pasting"> {
  return stage;
}

function showVisibleStatus(status: DictationStatus, dispatch: (action: Parameters<typeof appReducer>[1]) => void) {
  dispatch({ type: "statusChanged", status });
  return new Promise((resolve) => window.setTimeout(resolve, 120));
}
