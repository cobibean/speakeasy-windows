import { describe, expect, it } from "vitest";
import { selectRecordingMimeType } from "./audio";

describe("selectRecordingMimeType", () => {
  it("prefers webm opus when available", () => {
    const mediaRecorder = {
      isTypeSupported: (mimeType: string) => mimeType === "audio/webm;codecs=opus",
    };

    expect(selectRecordingMimeType(mediaRecorder)).toBe("audio/webm;codecs=opus");
  });

  it("falls back to a browser default when no preferred type is available", () => {
    const mediaRecorder = {
      isTypeSupported: () => false,
    };

    expect(selectRecordingMimeType(mediaRecorder)).toBe("");
  });
});
