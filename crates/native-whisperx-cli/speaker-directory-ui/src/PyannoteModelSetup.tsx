import { useMutation } from "@tanstack/react-query";
import { useEffect, useRef, useState } from "react";
import {
  archiveCommunityDiarizationResources,
  loadCommunityDiarizationResources,
} from "../vendor/pyannote-resources.js";
import { workbenchApi } from "./workbenchApi";

type PreparedModels = { bundlePath: string; vadBundlePath: string };
export function PyannoteModelSetup({
  onPrepared,
}: {
  onPrepared: (models: PreparedModels) => void;
}) {
  const tokenInput = useRef<HTMLInputElement>(null);
  const controller = useRef<AbortController | null>(null);
  const [progress, setProgress] = useState("");
  useEffect(() => () => controller.current?.abort(), []);
  const setup = useMutation({
    mutationFn: async () => {
      const operation = new AbortController();
      controller.current = operation;
      try {
        const bundle = await loadCommunityDiarizationResources({
          token: tokenInput.current?.value.trim() ?? "",
          signal: operation.signal,
          onProgress: ({ message }) => setProgress(message),
        });
        operation.signal.throwIfAborted();
        setProgress("Installing verified models in the local cache…");
        const archive = await archiveCommunityDiarizationResources(bundle);
        operation.signal.throwIfAborted();
        const result = await workbenchApi.installCommunityBundle(archive, operation.signal);
        operation.signal.throwIfAborted();
        return result;
      } finally {
        if (tokenInput.current) tokenInput.current.value = "";
        controller.current = null;
      }
    },
    onSuccess: (models) => {
      onPrepared(models);
      setProgress("Pyannote models are ready for VAD and diarization.");
    },
  });
  return (
    <section aria-label="Pyannote model setup">
      <p>
        Accept the{" "}
        <a
          href="https://huggingface.co/pyannote/speaker-diarization-community-1"
          target="_blank"
          rel="noreferrer"
        >
          community-1 model conditions
        </a>
        , then use a Hugging Face read token. Downloads run in your browser; verified models are
        stored in this app’s local cache.
      </p>
      <label>
        HF_TOKEN
        <input
          ref={tokenInput}
          type="password"
          autoComplete="off"
          spellCheck={false}
          disabled={setup.isPending}
        />
      </label>
      <p className="muted">
        The token is used only for Hugging Face downloads and is cleared after setup. It is never
        saved in presets or sent to the local app.
      </p>
      <button type="button" disabled={setup.isPending} onClick={() => setup.mutate()}>
        Prepare pyannote models
      </button>
      {setup.isPending ? (
        <button type="button" onClick={() => controller.current?.abort()}>
          Cancel model setup
        </button>
      ) : null}
      {progress ? <p role="status">{progress}</p> : null}
      {setup.error ? (
        <p role="alert">
          {setup.error.name === "AbortError" ? "Model setup cancelled." : setup.error.message}
        </p>
      ) : null}
    </section>
  );
}
