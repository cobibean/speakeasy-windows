import { invoke } from "@tauri-apps/api/core";
import { defaultSettings } from "../domain/state";
import type { AppSettings, PipelineResult } from "../domain/types";

const isTauriRuntime = "__TAURI_INTERNALS__" in window;

export async function loadSettings(): Promise<AppSettings> {
  if (!isTauriRuntime) {
    return defaultSettings;
  }

  return invoke<AppSettings>("get_settings");
}

export async function saveSettings(settings: AppSettings): Promise<AppSettings> {
  if (!isTauriRuntime) {
    return settings;
  }

  return invoke<AppSettings>("save_settings", { settings });
}

export async function startHoldToTalk(): Promise<void> {
  if (!isTauriRuntime) {
    return;
  }

  await invoke("start_hold_to_talk");
}

export async function stopAndRunPipeline(): Promise<PipelineResult> {
  if (!isTauriRuntime) {
    return {
      transcript: "Mock transcript from the Windows prototype loop.",
      polishedText: "Mock transcript from the Windows prototype loop.",
      pasted: false,
      usedCleanup: defaultSettings.cleanupEnabled,
      placeholder: true,
      completedStages: ["transcribing", "polishing"],
    };
  }

  return invoke<PipelineResult>("stop_and_run_pipeline");
}
