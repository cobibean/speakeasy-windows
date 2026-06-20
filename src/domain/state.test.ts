import { describe, expect, it } from "vitest";
import { appReducer, createInitialSnapshot, defaultSettings, statusLabel } from "./state";

describe("appReducer", () => {
  it("stores completed pipeline output without adding history", () => {
    const state = createInitialSnapshot();

    const next = appReducer(state, {
      type: "pipelineCompleted",
      result: {
        transcript: "hello there",
        polishedText: "Hello there.",
        pasted: true,
        usedCleanup: true,
        placeholder: true,
        completedStages: ["transcribing", "polishing", "pasting"],
      },
    });

    expect(next.status).toBe("complete");
    expect(next.lastTranscript).toBe("hello there");
    expect(next.lastPolishedText).toBe("Hello there.");
  });

  it("keeps settings narrow and local", () => {
    const state = createInitialSnapshot();
    const settings = { ...defaultSettings, cleanupEnabled: false, groqApiKey: "test-key-value" };

    const next = appReducer(state, { type: "settingsSaved", settings });

    expect(next.settings.cleanupEnabled).toBe(false);
    expect(next.settings.groqApiKey).toBe("test-key-value");
    expect(defaultSettings.groqApiKey).toBe("");
  });

  it("can keep microphone access failures distinct from generic errors", () => {
    const state = createInitialSnapshot();

    const next = appReducer(state, {
      type: "failed",
      status: "micAccessRequired",
      message: "Microphone access was denied.",
    });

    expect(next.status).toBe("micAccessRequired");
    expect(next.errorMessage).toBe("Microphone access was denied.");
  });
});

describe("statusLabel", () => {
  it("uses user-facing loop labels", () => {
    expect(statusLabel("listening")).toBe("Listening");
    expect(statusLabel("micAccessRequired")).toBe("Mic Access Required");
    expect(statusLabel("polishing")).toBe("Polishing");
  });
});
