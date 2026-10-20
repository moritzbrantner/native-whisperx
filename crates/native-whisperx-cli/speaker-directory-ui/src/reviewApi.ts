export type ReviewSegment = {
  id: string;
  text: string;
  startSeconds: number | null;
  endSeconds: number | null;
  speaker: string | null;
  wordCount: number;
  overlapsPrevious: boolean;
};
export type ReviewTranscript = { language: string | null; segments: ReviewSegment[] };
export type ReviewResult = {
  resultId: string;
  snapshotId: string;
  originalName: string | null;
  sourceName: string | null;
  mediaUrl: string | null;
  mediaType: string;
  mediaError: string | null;
  root: string;
  transcript: ReviewTranscript;
  sourceTranscript: ReviewTranscript | null;
  isTranslation: boolean;
  translationProvenance: {
    modelId: string | null;
    sourceLanguage: string | null;
    targetLanguage: string | null;
  } | null;
};
export type ReviewCorrection = {
  segmentId: string;
  text: string;
  startSeconds: number;
  endSeconds: number;
  speaker: string | null;
};
export type ReviewPreview = {
  previewId: string;
  transcript: ReviewTranscript;
  sourceTranscript: ReviewTranscript | null;
  clearedAlignment: string[];
  clearedSourceAlignment: string[];
};
export type ReviewReceipt = { resultId: string; outputs: { path: string; format: string }[] };
function object(raw: unknown): Record<string, unknown> {
  if (!raw || typeof raw !== "object" || Array.isArray(raw))
    throw new Error("Invalid Review response.");
  return Object.fromEntries(Object.entries(raw));
}
function string(raw: unknown): string {
  if (typeof raw !== "string") throw new Error("Invalid Review text.");
  return raw;
}
function nullableString(raw: unknown): string | null {
  return raw == null ? null : string(raw);
}
function boolean(raw: unknown): boolean {
  if (typeof raw !== "boolean") throw new Error("Invalid Review state.");
  return raw;
}
function array(raw: unknown): unknown[] {
  if (!Array.isArray(raw)) throw new Error("Invalid Review list.");
  return raw;
}
function seconds(raw: unknown): number | null {
  if (raw == null) return null;
  if (typeof raw !== "number" || !Number.isFinite(raw) || raw < 0)
    throw new Error("Invalid Review timing.");
  return raw;
}
export function parseReviewTranscript(raw: unknown): ReviewTranscript {
  const value = object(raw);
  return {
    language: nullableString(value.language),
    segments: array(value.segments).map((raw) => {
      const segment = object(raw);
      const wordCount = segment.wordCount;
      if (typeof wordCount !== "number" || !Number.isSafeInteger(wordCount) || wordCount < 0)
        throw new Error("Invalid alignment count.");
      return {
        id: string(segment.id),
        text: string(segment.text),
        startSeconds: seconds(segment.startSeconds),
        endSeconds: seconds(segment.endSeconds),
        speaker: nullableString(segment.speaker),
        wordCount,
        overlapsPrevious: boolean(segment.overlapsPrevious),
      };
    }),
  };
}
function result(raw: unknown): ReviewResult {
  const value = object(raw);
  const mediaUrl = nullableString(value.mediaUrl);
  if (mediaUrl && !/^\/api\/review\/media\/[a-f0-9]{64}$/.test(mediaUrl))
    throw new Error("Invalid local recording capability.");
  const provenance =
    value.translationProvenance == null ? null : object(value.translationProvenance);
  return {
    resultId: string(value.resultId),
    snapshotId: string(value.snapshotId),
    originalName: nullableString(value.originalName),
    sourceName: nullableString(value.sourceName),
    mediaUrl,
    mediaType: string(value.mediaType),
    mediaError: nullableString(value.mediaError),
    root: string(value.root),
    transcript: parseReviewTranscript(value.transcript),
    sourceTranscript:
      value.sourceTranscript == null ? null : parseReviewTranscript(value.sourceTranscript),
    isTranslation: boolean(value.isTranslation),
    translationProvenance: provenance
      ? {
          modelId: nullableString(provenance.modelId),
          sourceLanguage: nullableString(provenance.sourceLanguage),
          targetLanguage: nullableString(provenance.targetLanguage),
        }
      : null,
  };
}
async function request(path: string, body: unknown): Promise<unknown> {
  const encoded = JSON.stringify(body);
  if (new TextEncoder().encode(encoded).length > 1024 * 1024)
    throw new Error("Too many corrections for one copy. Save a smaller set of edits.");
  const response = await fetch(`/api/review/${path}`, {
    method: "POST",
    signal: AbortSignal.timeout(30_000),
    headers: {
      "Content-Type": "application/json",
      "X-Native-Whisperx-Session-Token": window.nativeWhisperxSessionToken ?? "",
    },
    body: encoded,
  });
  if (!response.ok) throw new Error(await response.text());
  return response.json();
}
export const reviewApi = {
  async open(resultId: string) {
    return result(await request("open", { resultId }));
  },
  async reset(resultId: string) {
    return result(await request("reset", { resultId }));
  },
  async recording(value: { resultId: string; path: string }) {
    return result(await request("recording", value));
  },
  async preview(value: {
    resultId: string;
    corrections: ReviewCorrection[];
    sourceCorrections: ReviewCorrection[];
  }): Promise<ReviewPreview> {
    const raw = object(await request("preview", value));
    return {
      previewId: string(raw.previewId),
      transcript: parseReviewTranscript(raw.transcript),
      sourceTranscript:
        raw.sourceTranscript == null ? null : parseReviewTranscript(raw.sourceTranscript),
      clearedAlignment: array(raw.clearedAlignment).map(string),
      clearedSourceAlignment: array(raw.clearedSourceAlignment).map(string),
    };
  },
  async save(value: {
    resultId: string;
    previewId: string;
    directory: string;
    basename: string;
    formats: string[];
    exportLayer: "transcript" | "source";
  }): Promise<ReviewReceipt> {
    const raw = object(await request("save", value));
    return {
      resultId: string(raw.resultId),
      outputs: array(raw.outputs).map((raw) => {
        const file = object(raw);
        return { path: string(file.path), format: string(file.format) };
      }),
    };
  },
};
