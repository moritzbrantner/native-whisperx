import { type SpeakerDirectoryState } from "./api";

export function record(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
const text = (value: unknown): value is string => typeof value === "string";
const optionalText = (value: unknown) => value === undefined || text(value);
const finite = (value: unknown): value is number =>
  typeof value === "number" && Number.isFinite(value);
const count = (value: unknown) => finite(value) && Number.isSafeInteger(value) && value >= 0;
const optionalNumber = (value: unknown) => value === undefined || finite(value);
const status = (value: unknown) => value === "valid" || value === "missing" || value === "invalid";
function metadata(value: unknown) {
  return record(value) && Object.values(value).every(text);
}
function span(value: unknown) {
  return (
    record(value) &&
    text(value.snippet) &&
    optionalNumber(value.startSeconds) &&
    optionalNumber(value.endSeconds)
  );
}
function file(value: unknown) {
  return (
    record(value) &&
    text(value.sourceFile) &&
    count(value.segmentCount) &&
    finite(value.totalDuration) &&
    Array.isArray(value.spans) &&
    value.spans.every(span)
  );
}
function speaker(value: unknown) {
  return (
    record(value) &&
    (value.kind === "enrolled" || value.kind === "anonymous") &&
    optionalText(value.profileId) &&
    optionalText(value.label) &&
    optionalText(value.anonymousLabel) &&
    Array.isArray(value.files) &&
    value.files.every(file)
  );
}
function profile(value: unknown) {
  return record(value) && text(value.id) && text(value.label) && metadata(value.metadata);
}
function traceError(value: unknown) {
  return record(value) && text(value.sourceFile) && text(value.message);
}
function state(value: unknown): value is SpeakerDirectoryState {
  if (!record(value) || !record(value.library) || !record(value.trace)) {
    return false;
  }
  return (
    (value.scope === "local" || value.scope === "global" || value.scope === "explicit") &&
    text(value.path) &&
    text(value.library.path) &&
    status(value.library.status) &&
    count(value.library.profileCount) &&
    optionalText(value.library.message) &&
    Array.isArray(value.profiles) &&
    value.profiles.every(profile) &&
    text(value.trace.path) &&
    status(value.trace.status) &&
    optionalText(value.trace.scanRoot) &&
    optionalText(value.trace.message) &&
    Array.isArray(value.trace.speakers) &&
    value.trace.speakers.every(speaker) &&
    Array.isArray(value.trace.errors) &&
    value.trace.errors.every(traceError)
  );
}
export function parseSpeakerDirectoryState(value: unknown): SpeakerDirectoryState {
  if (!state(value)) {
    throw new Error("Invalid Speaker Directory response.");
  }
  return value;
}
