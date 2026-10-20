import type { ReviewTranscript } from "./reviewApi";
import { reviewClock } from "./reviewTime";

export function TimedTranscript({
  transcript,
  fallbackText,
}: {
  transcript: ReviewTranscript | null;
  fallbackText: string;
}) {
  if (!transcript?.segments.length) return <p>{fallbackText || "No speech detected."}</p>;
  return (
    <ol className="timedTranscript" aria-label="Timestamped transcript">
      {transcript.segments.map((segment) => (
        <li key={segment.id}>
          <div className="segmentTiming">
            <span>
              {segment.startSeconds == null ? "Untimed" : reviewClock(segment.startSeconds)} →{" "}
              {segment.endSeconds == null ? "Untimed" : reviewClock(segment.endSeconds)}
            </span>
            {segment.speaker ? <strong>{segment.speaker}</strong> : null}
          </div>
          <p>{segment.text}</p>
        </li>
      ))}
    </ol>
  );
}
