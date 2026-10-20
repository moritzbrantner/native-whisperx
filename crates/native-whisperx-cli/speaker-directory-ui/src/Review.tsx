import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef, useState } from "react";
import { speakerDirectoryApi, type SpeakerDirectoryApi } from "./api";
import {
  reviewApi,
  type ReviewCorrection,
  type ReviewResult,
  type ReviewSegment,
  type ReviewTranscript,
} from "./reviewApi";
import { parseReviewTime, reviewClock } from "./reviewTime";
import { jobsApi } from "./jobsApi";
import { jobsKey } from "./Jobs";

type Draft = { text: string; start: string; end: string; speaker: string };
type Drafts = Record<string, Draft>;
type Layer = "transcript" | "source";
const emptyFingerprint = JSON.stringify({ transcript: {}, source: {} });
function fromSegment(segment: ReviewSegment): Draft {
  return {
    text: segment.text,
    start: segment.startSeconds == null ? "" : String(segment.startSeconds),
    end: segment.endSeconds == null ? "" : String(segment.endSeconds),
    speaker: segment.speaker ?? "",
  };
}
function edits(drafts: Drafts, original: ReviewTranscript | null): ReviewCorrection[] {
  return Object.entries(drafts).map(([id, draft]) => {
    if (!original?.segments.some((segment) => segment.id === id))
      throw new Error("Selected segment no longer exists. Reopen the result.");
    return {
      segmentId: id,
      text: draft.text,
      startSeconds: parseReviewTime(draft.start),
      endSeconds: parseReviewTime(draft.end),
      speaker: draft.speaker || null,
    };
  });
}
function reviewHref(resultId: string) {
  return `?view=review&result=${encodeURIComponent(resultId)}`;
}
export function Review({ api = speakerDirectoryApi }: { api?: SpeakerDirectoryApi }) {
  const resultId = new URLSearchParams(window.location.search).get("result");
  const query = useQuery({
    queryKey: ["review", resultId],
    queryFn: () => reviewApi.open(resultId ?? ""),
    enabled: !!resultId,
    retry: false,
    staleTime: Infinity,
    refetchOnWindowFocus: false,
  });
  if (!resultId) return <ReviewHome />;
  if (query.isPending)
    return (
      <main className="page reviewPage">
        <h1>Review &amp; Edit</h1>
        <p>Opening recorded result…</p>
      </main>
    );
  if (query.error || !query.data)
    return (
      <main className="page reviewPage">
        <h1>Review &amp; Edit</h1>
        <p role="alert">{query.error?.message ?? "Result is unavailable."}</p>
        <a href="?view=jobs">Open a completed result in Jobs</a>
        <p>
          <a href="?view=transcribe">Transcribe a recording</a>
        </p>
      </main>
    );
  return <ReviewWorkspace key={query.data.snapshotId} original={query.data} api={api} />;
}
function ReviewHome() {
  const query = useQuery({ queryKey: jobsKey, queryFn: jobsApi.history, retry: false });
  const [visibleCopies, setVisibleCopies] = useState(20);
  return (
    <main className="page reviewPage">
      <h1>Review &amp; Edit</h1>
      {query.data?.reviews.length ? (
        <section>
          <h2>Corrected copies</h2>
          <ul>
            {query.data.reviews.slice(0, visibleCopies).map((copy) => (
              <li key={copy.resultId}>
                <a href={reviewHref(copy.resultId)}>
                  {copy.outputs
                    .find((output) => output.format === "native-json")
                    ?.path.split(/[\\/]/)
                    .at(-1) ?? "Corrected copy"}
                </a>
              </li>
            ))}
          </ul>
          {query.data.reviews.length > visibleCopies ? (
            <button type="button" onClick={() => setVisibleCopies((count) => count + 20)}>
              Show older copies
            </button>
          ) : null}
        </section>
      ) : null}
      <p>
        Open a completed native result from <a href="?view=jobs">Jobs</a>, or{" "}
        <a href="?view=transcribe">transcribe a recording</a>.
      </p>
      {query.error ? <p role="alert">{query.error.message}</p> : null}
      <ul>
        {query.data?.jobs
          .flatMap((batch) => batch.files.map((file, index) => ({ batch, file, index })))
          .filter(
            ({ file }) =>
              file.status === "completed" &&
              file.outputs.some((output) => ["json", "native-json"].includes(output.format)),
          )
          .slice(0, 20)
          .map(({ batch, file, index }) => (
            <li key={`${batch.id}:${index}`}>
              <a href={reviewHref(`${batch.id}:${index}`)}>{file.input.split(/[\\/]/).at(-1)}</a>
            </li>
          ))}
      </ul>
    </main>
  );
}
function ReviewWorkspace({ original, api }: { original: ReviewResult; api: SpeakerDirectoryApi }) {
  const client = useQueryClient();
  const speakers = useQuery({
    queryKey: ["speaker-directory-state"],
    queryFn: () => api.getState(),
    retry: false,
  });
  const initial = new URLSearchParams(window.location.search);
  const [layer, setLayer] = useState<Layer>(
    initial.get("layer") === "source" && original.sourceTranscript ? "source" : "transcript",
  );
  const [selectedId, setSelectedId] = useState(
    initial.get("segment") ?? original.transcript.segments[0]?.id ?? "",
  );
  const [drafts, setDrafts] = useState<{ transcript: Drafts; source: Drafts }>({
    transcript: {},
    source: {},
  });
  const fingerprint = JSON.stringify(drafts);
  const [savedFingerprint, setSavedFingerprint] = useState(emptyFingerprint);
  const dirty = fingerprint !== savedFingerprint;
  const [directory, setDirectory] = useState(".");
  const [basename, setBasename] = useState(
    `${(original.originalName ?? "result").replace(/(?:\.native)?\.json$/i, "")}.corrected`,
  );
  const [formats, setFormats] = useState(["json", "srt"]);
  const [exportLayer, setExportLayer] = useState<Layer>("transcript");
  const [recordingPath, setRecordingPath] = useState("");
  const [mediaFailure, setMediaFailure] = useState("");
  const [playhead, setPlayhead] = useState(0);
  const player = useRef<HTMLMediaElement | null>(null);
  const buttons = useRef(new Map<string, HTMLButtonElement>());
  const lastPlaying = useRef<string | null>(null);
  const preview = useMutation({
    mutationFn: async (value: { fingerprint: string; drafts: typeof drafts }) => ({
      fingerprint: value.fingerprint,
      preview: await reviewApi.preview({
        resultId: original.resultId,
        corrections: edits(value.drafts.transcript, original.transcript),
        sourceCorrections: edits(value.drafts.source, original.sourceTranscript),
      }),
    }),
  });
  const validPreview = preview.data?.fingerprint === fingerprint ? preview.data.preview : null;
  const save = useMutation({
    mutationFn: async (value: { fingerprint: string; previewId: string }) =>
      reviewApi.save({
        resultId: original.resultId,
        previewId: value.previewId,
        directory,
        basename,
        formats,
        exportLayer,
      }),
    onSuccess: (_receipt, variables) => {
      setSavedFingerprint(variables.fingerprint);
      void client.invalidateQueries({ queryKey: jobsKey });
    },
  });
  const reset = useMutation({
    mutationFn: () => reviewApi.reset(original.resultId),
    onSuccess: () => {
      setDrafts({ transcript: {}, source: {} });
      setSavedFingerprint(emptyFingerprint);
      preview.reset();
      save.reset();
    },
  });
  const locate = useMutation({
    mutationFn: () => reviewApi.recording({ resultId: original.resultId, path: recordingPath }),
    onSuccess: (value) => {
      setMediaFailure("");
      client.setQueryData(["review", original.resultId], value);
    },
  });
  useEffect(() => {
    if (!dirty) return;
    const warn = (event: BeforeUnloadEvent) => {
      event.preventDefault();
      event.returnValue = "";
    };
    window.addEventListener("beforeunload", warn);
    return () => window.removeEventListener("beforeunload", warn);
  }, [dirty]);
  const current =
    layer === "source"
      ? (validPreview?.sourceTranscript ?? original.sourceTranscript)
      : (validPreview?.transcript ?? original.transcript);
  const segments = current?.segments ?? [];
  const originalLayer = layer === "source" ? original.sourceTranscript : original.transcript;
  const active = new Set(
    segments
      .filter(
        (segment) =>
          segment.startSeconds != null &&
          segment.endSeconds != null &&
          playhead >= segment.startSeconds &&
          playhead < segment.endSeconds,
      )
      .map((segment) => segment.id),
  );
  const profiles = speakers.data?.profiles ?? [];
  const known = new Map<string, string>();
  for (const segment of [
    ...original.transcript.segments,
    ...(original.sourceTranscript?.segments ?? []),
  ])
    if (segment.speaker) known.set(segment.speaker, segment.speaker);
  for (const profile of profiles) {
    known.set(profile.label, `${profile.label} (${profile.metadata.status ?? "unclassified"})`);
    if (known.has(profile.id))
      known.set(
        profile.id,
        `${profile.label} (${profile.metadata.status ?? "unclassified"}; stored ID)`,
      );
  }
  for (const collection of Object.values(drafts))
    for (const draft of Object.values(collection))
      if (draft.speaker && !known.has(draft.speaker)) known.set(draft.speaker, draft.speaker);
  const error = preview.error ?? save.error ?? reset.error ?? locate.error ?? speakers.error;
  function select(segment: ReviewSegment) {
    setSelectedId(segment.id);
    if (player.current && segment.startSeconds != null) {
      player.current.currentTime = segment.startSeconds;
      setPlayhead(player.current.currentTime);
    }
    const url = new URL(window.location.href);
    url.searchParams.set("segment", segment.id);
    url.searchParams.set("layer", layer);
    window.history.replaceState(null, "", url);
  }
  function update(segment: ReviewSegment, field: keyof Draft, value: string) {
    const baseline = originalLayer?.segments.find((original) => original.id === segment.id);
    if (!baseline) return;
    setDrafts((previous) => {
      const draft = { ...(previous[layer][segment.id] ?? fromSegment(baseline)), [field]: value };
      const next = { ...previous[layer] };
      const unchanged = JSON.stringify(draft) === JSON.stringify(fromSegment(baseline));
      if (unchanged) delete next[segment.id];
      else next[segment.id] = draft;
      return { ...previous, [layer]: next };
    });
    preview.reset();
    save.reset();
  }
  function loaded() {
    const segment = segments.find((segment) => segment.id === selectedId);
    if (player.current && segment?.startSeconds != null) {
      player.current.currentTime = segment.startSeconds;
      setPlayhead(player.current.currentTime);
    }
  }
  const mediaProps = {
    "aria-label": "Source recording",
    controls: true,
    src: original.mediaUrl ?? undefined,
    onLoadedMetadata: loaded,
    onTimeUpdate: () => {
      if (player.current) {
        const time = player.current.currentTime;
        setPlayhead(time);
        const currentSegment = segments.find(
          (segment) =>
            segment.startSeconds != null &&
            segment.endSeconds != null &&
            time >= segment.startSeconds &&
            time < segment.endSeconds,
        );
        if (currentSegment && lastPlaying.current !== currentSegment.id) {
          lastPlaying.current = currentSegment.id;
          if (!["TEXTAREA", "INPUT", "SELECT"].includes(document.activeElement?.tagName ?? ""))
            buttons.current.get(currentSegment.id)?.scrollIntoView?.({ block: "nearest" });
        }
      }
    },
    onError: () =>
      setMediaFailure(
        "This browser could not play the recording. Locate a compatible local audio/video file below.",
      ),
  };
  return (
    <main className="page reviewPage">
      <header className="reviewHeader">
        <div>
          <h1>Review &amp; Edit</h1>
          <p className="path">{original.originalName}</p>
        </div>
        <p role="status">{dirty ? "Unsaved changes" : "No unsaved changes"}</p>
      </header>
      {error ? <p role="alert">{error.message}</p> : null}
      <div className="reviewActions">
        <button
          type="button"
          disabled={reset.isPending || save.isPending || (!dirty && !validPreview)}
          onClick={() => reset.mutate()}
        >
          Discard changes
        </button>
        <a href="?view=jobs">Jobs</a>
      </div>
      <div className="reviewWorkspace">
        <section className="reviewMedia" aria-label="Recording playback">
          <h2>{original.sourceName ?? "Source recording"}</h2>
          {original.mediaUrl ? (
            original.mediaType.startsWith("video/") ? (
              <video
                {...mediaProps}
                ref={(node) => {
                  player.current = node;
                }}
              />
            ) : (
              <audio
                {...mediaProps}
                ref={(node) => {
                  player.current = node;
                }}
              />
            )
          ) : null}
          <p>Playback {reviewClock(playhead)}</p>
          {original.mediaError || mediaFailure ? (
            <p role="alert">{original.mediaError ?? mediaFailure}</p>
          ) : null}
          <details open={!!original.mediaError || !!mediaFailure}>
            <summary>Locate source recording</summary>
            <label>
              Matching local recording path
              <input
                value={recordingPath}
                onChange={(event) => setRecordingPath(event.currentTarget.value)}
              />
            </label>
            <button
              type="button"
              disabled={!recordingPath.trim() || locate.isPending}
              onClick={() => locate.mutate()}
            >
              Use recording
            </button>
            <p className="muted path">
              Paths must stay inside {original.root}. The recording file is never modified.
            </p>
          </details>
          {original.translationProvenance ? (
            <p className="muted">
              Translation: {original.translationProvenance.sourceLanguage ?? "source"} →{" "}
              {original.translationProvenance.targetLanguage ?? "target"} ·{" "}
              {original.translationProvenance.modelId ?? "recorded model"}
            </p>
          ) : null}
          {original.isTranslation && !original.sourceTranscript ? (
            <p>
              This older translated result has no retained source transcript.{" "}
              <a href="?view=transcribe">Run Transcribe again</a> to retain both languages.
            </p>
          ) : null}
        </section>
        <section className="reviewTranscript" aria-label="Timed transcript">
          {original.sourceTranscript ? (
            <div className="reviewLayers" aria-label="Transcript layer">
              {(["transcript", "source"] satisfies Layer[]).map((value) => (
                <button
                  type="button"
                  key={value}
                  aria-pressed={layer === value}
                  onClick={() => {
                    setLayer(value);
                    const url = new URL(window.location.href);
                    url.searchParams.set("layer", value);
                    window.history.replaceState(null, "", url);
                  }}
                >
                  {value === "source" ? "Source transcript" : "Translation"}
                </button>
              ))}
            </div>
          ) : null}
          <p className="muted">
            Select a segment to seek and edit. Arrow keys move between segment buttons. Times accept
            exact seconds or HH:MM:SS.mmm.
          </p>
          {segments.length > 0 &&
          selectedId &&
          !segments.some((segment) => segment.id === selectedId) ? (
            <p>The selected segment was not found. Choose another segment below.</p>
          ) : null}
          {!segments.length ? (
            <p>
              No transcript segments are available.{" "}
              <a href="?view=jobs">Choose another result in Jobs</a> or{" "}
              <a href="?view=transcribe">transcribe a recording</a>.
            </p>
          ) : null}
          {segments.map((segment, index) => {
            const baseline =
              originalLayer?.segments.find((original) => original.id === segment.id) ?? segment;
            const draft = drafts[layer][segment.id] ?? fromSegment(baseline);
            const selected = selectedId === segment.id;
            return (
              <article
                className={`reviewSegment${selected ? " isSelected" : ""}${active.has(segment.id) ? " isPlaying" : ""}`}
                key={segment.id}
              >
                <button
                  type="button"
                  ref={(node) => {
                    if (node) buttons.current.set(segment.id, node);
                    else buttons.current.delete(segment.id);
                  }}
                  tabIndex={
                    selected || (index === 0 && !segments.some((item) => item.id === selectedId))
                      ? 0
                      : -1
                  }
                  aria-label={`Seek to segment ${index + 1}`}
                  aria-current={active.has(segment.id) ? "time" : undefined}
                  onClick={() => select(segment)}
                  onKeyDown={(event) => {
                    if (event.altKey || event.ctrlKey || event.metaKey) return;
                    const next =
                      event.key === "ArrowDown"
                        ? segments[index + 1]
                        : event.key === "ArrowUp"
                          ? segments[index - 1]
                          : event.key === "Home"
                            ? segments[0]
                            : event.key === "End"
                              ? segments.at(-1)
                              : undefined;
                    if (next) {
                      event.preventDefault();
                      select(next);
                      buttons.current.get(next.id)?.focus();
                    }
                  }}
                >
                  Segment {index + 1} ·{" "}
                  {segment.startSeconds == null ? "Untimed" : reviewClock(segment.startSeconds)} →{" "}
                  {segment.endSeconds == null ? "Untimed" : reviewClock(segment.endSeconds)} ·{" "}
                  {draft.speaker || "Unassigned"}
                </button>
                {segment.overlapsPrevious ? (
                  <p className="reviewOverlap">
                    Overlapping speech with the previous segment; source order is preserved.
                  </p>
                ) : null}
                {selected ? (
                  <>
                    <label>
                      Segment {index + 1} text
                      <textarea
                        aria-label={`Segment ${index + 1} text`}
                        rows={3}
                        value={draft.text}
                        onChange={(event) => update(segment, "text", event.currentTarget.value)}
                      />
                    </label>
                    <div className="reviewExactTimes">
                      <label>
                        Segment {index + 1} start (seconds or timecode)
                        <input
                          inputMode="decimal"
                          value={draft.start}
                          onChange={(event) => update(segment, "start", event.currentTarget.value)}
                        />
                      </label>
                      <label>
                        Segment {index + 1} end (seconds or timecode)
                        <input
                          inputMode="decimal"
                          value={draft.end}
                          onChange={(event) => update(segment, "end", event.currentTarget.value)}
                        />
                      </label>
                    </div>
                    <label>
                      Segment {index + 1} speaker annotation
                      <select
                        aria-label={`Segment ${index + 1} speaker annotation`}
                        value={draft.speaker}
                        onChange={(event) => update(segment, "speaker", event.currentTarget.value)}
                      >
                        <option value="">Unassigned</option>
                        {[...known].map(([value, label]) => (
                          <option key={value} value={value}>
                            {label}
                          </option>
                        ))}
                      </select>
                    </label>
                    <p className="muted">
                      Annotation does not confirm a Speaker Profile.{" "}
                      <a href="?view=speakers" target="_blank" rel="noreferrer">
                        Confirm or correct identity in Speakers
                      </a>
                      .
                    </p>
                  </>
                ) : (
                  <p>{draft.text}</p>
                )}
              </article>
            );
          })}
        </section>
      </div>
      <section className="reviewExport" aria-label="Corrected copy">
        <h2>Preview and save</h2>
        <button
          type="button"
          disabled={preview.isPending || !segments.length}
          onClick={() => {
            save.reset();
            preview.mutate({ fingerprint, drafts });
          }}
        >
          Preview changes
        </button>
        {validPreview ? (
          <>
            <p role="status">Preview validated. Segment order is unchanged.</p>
            {validPreview.clearedAlignment.length || validPreview.clearedSourceAlignment.length ? (
              <p>
                Word alignment cleared for manually edited text/timing. Neighboring segments retain
                their alignment.
              </p>
            ) : null}
          </>
        ) : null}
        <p className="muted">
          A new canonical Native JSON copy always retains source/translation metadata. Text and
          subtitle exports use{" "}
          {exportLayer === "source"
            ? "Source"
            : original.isTranslation
              ? "Translation"
              : "Transcript"}
          . Existing files and the original recording/result are never overwritten.
        </p>
        {original.sourceTranscript ? (
          <label>
            Text/subtitle export layer
            <select
              aria-label="Text/subtitle export layer"
              value={exportLayer}
              onChange={(event) => {
                const value = event.currentTarget.value;
                if (value === "source" || value === "transcript") setExportLayer(value);
              }}
            >
              <option value="transcript">Translation</option>
              <option value="source">Source transcript</option>
            </select>
          </label>
        ) : null}
        <div className="reviewExportFields">
          <label>
            Output folder
            <input
              value={directory}
              onChange={(event) => setDirectory(event.currentTarget.value)}
            />
          </label>
          <label>
            Corrected-copy name
            <input value={basename} onChange={(event) => setBasename(event.currentTarget.value)} />
          </label>
        </div>
        <fieldset>
          <legend>Additional exports</legend>
          {[
            ["json", "WhisperX JSON"],
            ["txt", "TXT"],
            ["srt", "SRT"],
            ["vtt", "WebVTT"],
            ["tsv", "TSV"],
            ["aud", "Audacity"],
          ].map(([format, label]) =>
            format && label ? (
              <label key={format}>
                <input
                  type="checkbox"
                  checked={formats.includes(format)}
                  onChange={(event) =>
                    setFormats((previous) =>
                      event.currentTarget.checked
                        ? [...previous, format]
                        : previous.filter((value) => value !== format),
                    )
                  }
                />
                {label}
              </label>
            ) : null,
          )}
        </fieldset>
        <button
          type="button"
          disabled={!validPreview || save.isPending || save.isSuccess}
          onClick={() => {
            if (validPreview) save.mutate({ fingerprint, previewId: validPreview.previewId });
          }}
        >
          Save corrected copy
        </button>
        {save.data ? (
          <div role="status">
            <p>
              Corrected copy saved. <a href={reviewHref(save.data.resultId)}>Open corrected copy</a>
            </p>
            <ul>
              {save.data.outputs.map((output) => (
                <li className="path" key={output.path}>
                  {output.path}
                </li>
              ))}
            </ul>
          </div>
        ) : null}
      </section>
    </main>
  );
}
