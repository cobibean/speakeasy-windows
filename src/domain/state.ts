import type { AppSettings, AppSnapshot, DictationStatus, PipelineResult } from "./types";

export const defaultSettings: AppSettings = {
  holdToTalkHotkey: "Ctrl+Alt+Space",
  cleanupEnabled: true,
  cleanupStrength: "standard",
  pasteAfterTranscription: true,
  providerMode: "mock",
};

export const createInitialSnapshot = (settings: AppSettings = defaultSettings): AppSnapshot => ({
  status: "idle",
  settings,
  lastTranscript: "",
  lastPolishedText: "",
  errorMessage: null,
});

export type AppAction =
  | { type: "settingsLoaded"; settings: AppSettings }
  | { type: "settingsSaved"; settings: AppSettings }
  | { type: "statusChanged"; status: DictationStatus }
  | { type: "pipelineCompleted"; result: PipelineResult }
  | { type: "failed"; message: string }
  | { type: "resetError" };

export function appReducer(state: AppSnapshot, action: AppAction): AppSnapshot {
  switch (action.type) {
    case "settingsLoaded":
    case "settingsSaved":
      return { ...state, settings: action.settings, errorMessage: null };
    case "statusChanged":
      return { ...state, status: action.status, errorMessage: null };
    case "pipelineCompleted":
      return {
        ...state,
        status: "complete",
        lastTranscript: action.result.transcript,
        lastPolishedText: action.result.polishedText,
        errorMessage: null,
      };
    case "failed":
      return { ...state, status: "error", errorMessage: action.message };
    case "resetError":
      return { ...state, errorMessage: null, status: state.status === "error" ? "idle" : state.status };
    default:
      return state;
  }
}

export function statusLabel(status: DictationStatus): string {
  const labels: Record<DictationStatus, string> = {
    idle: "Ready",
    micAccessRequired: "Mic Access Required",
    listening: "Listening",
    transcribing: "Transcribing",
    polishing: "Polishing",
    pasting: "Pasting",
    complete: "Complete",
    error: "Error",
  };

  return labels[status];
}
