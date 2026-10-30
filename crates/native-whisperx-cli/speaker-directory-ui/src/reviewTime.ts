/** Exact-entry parsing only. Playback and canonical server validation own timing. */
export function parseReviewTime(raw: string): number {
  const value = raw.trim();
  const failure = () => new Error("Enter a finite nonnegative time in seconds or HH:MM:SS.mmm.");
  if (!value) throw failure();
  const parts = value.split(":");
  let seconds: number;
  if (parts.length === 1) {
    if (!/^(?:\d+(?:\.\d*)?|\.\d+)(?:e[+-]?\d+)?$/i.test(value)) throw failure();
    seconds = Number(value);
  } else if (parts.length === 2 || parts.length === 3) {
    const last = parts.at(-1);
    if (!last || !/^\d+(?:\.\d*)?$/.test(last)) throw failure();
    const whole = parts.slice(0, -1);
    if (whole.some((part) => !/^\d+$/.test(part))) throw failure();
    const numbers = whole.map(Number);
    const tail = Number(last);
    if (tail >= 60) throw failure();
    if (parts.length === 3 && (numbers[1] ?? 0) >= 60) throw failure();
    seconds =
      parts.length === 3
        ? (numbers[0] ?? 0) * 3600 + (numbers[1] ?? 0) * 60 + tail
        : (numbers[0] ?? 0) * 60 + tail;
  } else throw failure();
  if (!Number.isFinite(seconds) || seconds < 0) throw failure();
  return seconds;
}
export function reviewClock(seconds: number): string {
  if (!Number.isFinite(seconds)) return "—";
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const rest = (seconds % 60).toFixed(3).padStart(6, "0");
  return `${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}:${rest}`;
}
