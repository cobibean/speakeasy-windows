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
  groqApiKey: string;
  sttModel: string;
  cleanupEnabled: boolean;
  cleanupStrength: CleanupStrength;
  cleanupModel: string;
  pasteAfterTranscription: boolean;
  providerMode: "mock" | "groq";
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

export interface CapturedAudio {
  audioBase64: string;
  mimeType: string;
}
