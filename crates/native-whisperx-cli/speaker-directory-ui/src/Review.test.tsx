import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { Review } from "./Review";
import { mockSpeakerDirectoryApi } from "./mockSpeakerDirectoryApi";

const original = {
  resultId: "fixture:0",
  snapshotId: "snapshot",
  originalName: "original.native.json",
  sourceName: "recording.wav",
  mediaUrl: `/api/review/media/${"a".repeat(64)}`,
  mediaType: "audio/wav",
  mediaError: null,
  isTranslation: false,
  sourceTranscript: null,
  translationProvenance: null,
  root: "/recordings",
  transcript: {
    language: "en",
    segments: [
      {
        id: "0",
        text: "First segment",
        startSeconds: 0,
        endSeconds: 1.2,
        speaker: "SPEAKER_00",
        wordCount: 2,
        overlapsPrevious: false,
      },
      {
        id: "1",
        text: "Second segment",
        startSeconds: 1.35,
        endSeconds: 2.4,
        speaker: "SPEAKER_01",
        wordCount: 2,
        overlapsPrevious: false,
      },
    ],
  },
};
afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
  window.history.replaceState(null, "", "/");
});
it("seeks segment two, retains exact timing drafts, previews and saves a new copy", async () => {
  const previews: unknown[] = [];
  const saves: unknown[] = [];
  window.history.replaceState(null, "", "/?view=review&result=fixture%3A0&segment=1");
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      let value: unknown = original;
      if (path.endsWith("/preview")) {
        previews.push(JSON.parse(String(init?.body)));
        value = {
          previewId: "reviewed",
          transcript: {
            ...original.transcript,
            segments: [
              original.transcript.segments[0],
              {
                ...original.transcript.segments[1],
                text: "Corrected second",
                startSeconds: 1.5,
                endSeconds: 2.8,
                speaker: "Speaker A",
                wordCount: 0,
              },
            ],
          },
          sourceTranscript: null,
          clearedAlignment: ["1"],
          clearedSourceAlignment: [],
        };
      }
      if (path.endsWith("/save")) {
        saves.push(JSON.parse(String(init?.body)));
        value = {
          resultId: "copy:new",
          outputs: [{ format: "native-json", path: "/recordings/corrected.native.json" }],
        };
      }
      return new Response(JSON.stringify(value));
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Review api={mockSpeakerDirectoryApi} />
    </QueryClientProvider>,
  );
  await screen.findByLabelText("Segment 2 text");
  const player = screen.getByLabelText<HTMLAudioElement>("Source recording");
  fireEvent.loadedMetadata(player);
  expect(player.currentTime).toBe(1.35);
  fireEvent.change(screen.getByLabelText("Segment 2 text"), {
    target: { value: "Corrected second" },
  });
  fireEvent.change(screen.getByLabelText("Segment 2 start (seconds or timecode)"), {
    target: { value: "1.500" },
  });
  expect(screen.getByLabelText("Segment 2 start (seconds or timecode)")).toHaveValue("1.500");
  fireEvent.change(screen.getByLabelText("Segment 2 end (seconds or timecode)"), {
    target: { value: "00:00:02.800" },
  });
  fireEvent.change(screen.getByLabelText("Segment 2 speaker annotation"), {
    target: { value: "Speaker A" },
  });
  expect(screen.getByText("Unsaved changes")).toBeVisible();
  fireEvent.click(screen.getByRole("button", { name: "Preview changes" }));
  await screen.findByText(/Word alignment cleared/);
  expect(previews).toEqual([
    {
      resultId: "fixture:0",
      corrections: [
        {
          segmentId: "1",
          text: "Corrected second",
          startSeconds: 1.5,
          endSeconds: 2.8,
          speaker: "Speaker A",
        },
      ],
      sourceCorrections: [],
    },
  ]);
  fireEvent.change(screen.getByLabelText("Corrected-copy name"), {
    target: { value: "corrected" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Save corrected copy" }));
  await screen.findByRole("link", { name: "Open corrected copy" });
  expect(saves).toHaveLength(1);
  expect(saves[0]).toMatchObject({
    resultId: "fixture:0",
    previewId: "reviewed",
    basename: "corrected",
  });
  await waitFor(() => expect(screen.queryByText("Unsaved changes")).toBeNull());
  expect(window.location.search).not.toContain("recording.wav");
});

it("supports keyboard segment navigation and discards invalid exact-time drafts", async () => {
  let previewRequests = 0;
  window.history.replaceState(null, "", "/?view=review&result=fixture%3A0");
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string) => {
      if (path.endsWith("/preview")) previewRequests += 1;
      return new Response(JSON.stringify(original));
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Review api={mockSpeakerDirectoryApi} />
    </QueryClientProvider>,
  );
  const first = await screen.findByRole("button", { name: "Seek to segment 1" });
  first.focus();
  fireEvent.keyDown(first, { key: "ArrowDown" });
  expect(screen.getByRole("button", { name: "Seek to segment 2" })).toHaveFocus();
  expect(new URLSearchParams(window.location.search).get("segment")).toBe("1");
  const player = screen.getByLabelText<HTMLAudioElement>("Source recording");
  player.currentTime = 0.5;
  fireEvent.timeUpdate(player);
  expect(first).toHaveAttribute("aria-current", "time");
  fireEvent.change(screen.getByLabelText("Segment 2 start (seconds or timecode)"), {
    target: { value: "not-a-time" },
  });
  expect(screen.getByLabelText("Segment 2 start (seconds or timecode)")).toHaveValue("not-a-time");
  fireEvent.click(screen.getByRole("button", { name: "Preview changes" }));
  await screen.findByText(/Enter a finite nonnegative time/);
  expect(previewRequests).toBe(0);
  expect(screen.getByRole("button", { name: "Save corrected copy" })).toBeDisabled();
  fireEvent.click(screen.getByRole("button", { name: "Discard changes" }));
  await waitFor(() =>
    expect(screen.getByLabelText("Segment 2 start (seconds or timecode)")).toHaveValue("1.35"),
  );
  expect(screen.queryByText("Unsaved changes")).toBeNull();
});

it("keeps translation and source drafts separate and exports the explicitly chosen language", async () => {
  const previews: unknown[] = [];
  const saves: unknown[] = [];
  const bilingual = {
    ...original,
    isTranslation: true,
    sourceTranscript: original.transcript,
    translationProvenance: { modelId: "fixture/en-de", sourceLanguage: "en", targetLanguage: "de" },
    transcript: {
      ...original.transcript,
      language: "de",
      segments: original.transcript.segments.map((segment) => ({
        ...segment,
        text: `Translation ${segment.id}`,
      })),
    },
  };
  window.history.replaceState(null, "", "/?view=review&result=fixture%3A0");
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      let value: unknown = bilingual;
      if (path.endsWith("/preview")) {
        previews.push(JSON.parse(String(init?.body)));
        value = {
          previewId: `preview-${previews.length}`,
          transcript: {
            ...bilingual.transcript,
            segments: bilingual.transcript.segments.map((segment) =>
              segment.id === "0"
                ? { ...segment, text: "Translation correction", wordCount: 0 }
                : segment,
            ),
          },
          sourceTranscript: {
            ...original.transcript,
            segments: original.transcript.segments.map((segment) =>
              segment.id === "0" && previews.length > 1
                ? { ...segment, text: "Source correction", wordCount: 0 }
                : segment,
            ),
          },
          clearedAlignment: ["0"],
          clearedSourceAlignment: previews.length > 1 ? ["0"] : [],
        };
      }
      if (path.endsWith("/save")) {
        saves.push(JSON.parse(String(init?.body)));
        value = {
          resultId: "copy:bilingual",
          outputs: [
            { format: "native-json", path: "/recordings/bilingual.native.json" },
            { format: "srt", path: "/recordings/bilingual.srt" },
          ],
        };
      }
      return new Response(JSON.stringify(value));
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Review api={mockSpeakerDirectoryApi} />
    </QueryClientProvider>,
  );
  await screen.findByLabelText("Segment 1 text");
  fireEvent.change(screen.getByLabelText("Segment 1 text"), {
    target: { value: "Translation correction" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Preview changes" }));
  await screen.findByText("Preview validated. Segment order is unchanged.");
  expect(previews[0]).toMatchObject({
    sourceCorrections: [],
    corrections: [{ segmentId: "0", text: "Translation correction" }],
  });
  fireEvent.click(screen.getByRole("button", { name: "Source transcript" }));
  expect(screen.getByLabelText("Segment 1 text")).toHaveValue("First segment");
  fireEvent.change(screen.getByLabelText("Segment 1 text"), {
    target: { value: "Source correction" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Preview changes" }));
  await screen.findByText("Preview validated. Segment order is unchanged.");
  expect(previews[1]).toMatchObject({
    sourceCorrections: [{ segmentId: "0", text: "Source correction" }],
    corrections: [{ segmentId: "0", text: "Translation correction" }],
  });
  fireEvent.click(screen.getByRole("button", { name: "Translation" }));
  expect(screen.getByLabelText("Segment 1 text")).toHaveValue("Translation correction");
  fireEvent.change(screen.getByLabelText("Text/subtitle export layer"), {
    target: { value: "source" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Save corrected copy" }));
  await screen.findByRole("link", { name: "Open corrected copy" });
  expect(saves[0]).toMatchObject({ exportLayer: "source" });
});
