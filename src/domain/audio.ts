const preferredRecordingTypes = ["audio/webm;codecs=opus", "audio/webm", "audio/mp4", "audio/ogg"];

export function selectRecordingMimeType(mediaRecorder: Pick<typeof MediaRecorder, "isTypeSupported">): string {
  return preferredRecordingTypes.find((mimeType) => mediaRecorder.isTypeSupported(mimeType)) ?? "";
}
