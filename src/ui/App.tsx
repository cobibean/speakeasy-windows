import { useEffect, useMemo, useReducer, useState } from "react";
import { createInitialSnapshot, appReducer, statusLabel } from "../domain/state";
import type { AppSettings, CleanupStrength, DictationStatus, PipelineStage } from "../domain/types";
import { loadSettings, saveSettings, startHoldToTalk, stopAndRunPipeline } from "../tauri/commands";

const cleanupOptions: CleanupStrength[] = ["light", "standard", "strong"];

export function App() {
  const [state, dispatch] = useReducer(appReducer, undefined, createInitialSnapshot);
  const [draftSettings, setDraftSettings] = useState<AppSettings>(state.settings);
  const canStart = state.status === "idle" || state.status === "complete" || state.status === "error";
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
    return "Press Start, speak a short phrase, then Stop. This prototype returns safe placeholder text until Windows audio, STT, cleanup, and paste are wired.";
  }, [state.lastPolishedText, state.lastTranscript]);

  async function handleStart() {
    dispatch({ type: "statusChanged", status: "listening" });
    try {
      await startHoldToTalk();
    } catch (error) {
      dispatch({ type: "failed", message: toErrorMessage(error) });
    }
  }

  async function handleStop() {
    try {
      dispatch({ type: "statusChanged", status: "transcribing" });
      const result = await stopAndRunPipeline();
      for (const stage of result.completedStages) {
        const status = statusForStage(stage);
        if (status !== "transcribing") {
          await showVisibleStatus(status, dispatch);
        }
      }
      dispatch({ type: "pipelineCompleted", result });
    } catch (error) {
      dispatch({ type: "failed", message: toErrorMessage(error) });
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
          <div className="notice">Prototype mode uses local placeholders. No provider key is stored in this repo.</div>
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

          <label className="toggle-row">
            <input
              type="checkbox"
              checked={draftSettings.pasteAfterTranscription}
              onChange={(event) =>
                setDraftSettings({ ...draftSettings, pasteAfterTranscription: event.target.checked })
              }
            />
            <span>Paste immediately after transcription</span>
          </label>

          <button type="button" className="secondary-button full-width" onClick={handleSaveSettings}>
            Save settings
          </button>
        </section>

        <footer className="platform-notes">
          Windows native work still requires human testing for global hotkey, microphone capture, active-window paste,
          tray behavior, installer, and signing.
        </footer>
      </section>
    </main>
  );
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
