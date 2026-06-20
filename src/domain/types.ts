export type DictationStatus =
  | "idle"
  | "micAccessRequired"
  | "listening"
  | "transcribing"
  | "polishing"
  | "pasting"
  | "complete"
  | "error";

export type CleanupStrength = "light" | "standard" | "strong";

export interface AppSettings {
  holdToTalkHotkey: string;
  cleanupEnabled: boolean;
  cleanupStrength: CleanupStrength;
  pasteAfterTranscription: boolean;
  providerMode: "mock" | "manual";
}

export interface AppSnapshot {
  status: DictationStatus;
  settings: AppSettings;
  lastTranscript: string;
  lastPolishedText: string;
  errorMessage: string | null;
}

export interface PipelineResult {
  transcript: string;
  polishedText: string;
  pasted: boolean;
  usedCleanup: boolean;
  placeholder: boolean;
  completedStages: PipelineStage[];
}

export type PipelineStage = "transcribing" | "polishing" | "pasting";
