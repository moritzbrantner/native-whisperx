import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useId, useRef, useState, type ReactNode } from "react";

import { LibraryPathPicker } from "./LibraryPathPicker";
import { speakerDirectoryApi, type SpeakerDirectoryApi } from "./api";
import {
  speakerLibraryApi,
  type DirectoryChoice,
  type LibraryApplied,
  type LibraryPreview,
  type LibraryPreviewRequest,
} from "./speakerLibraryApi";

const stateKey = ["speaker-directory-state"];
type Action = "load" | "new" | "export" | "import";
const titles: Record<Action, string> = {
  load: "Load speaker index",
  new: "New speaker index",
  export: "Save speaker index",
  import: "Import speaker index",
};
const confirmationLabels = {
  new: "Create empty library",
  export: "Save copy",
  import: "Import library",
};

function unreachable(_value: never): never {
  throw new Error("Unsupported Speaker Library operation.");
}

export function SpeakerIndexControls({
  api = speakerDirectoryApi,
  manageHref,
  onIndexReplaced,
}: {
  api?: SpeakerDirectoryApi;
  manageHref?: string;
  onIndexReplaced?: () => void;
}) {
  const client = useQueryClient();
  const state = useQuery({
    queryKey: stateKey,
    queryFn: () => api.getState(),
    refetchOnMount: false,
  });
  const [action, setAction] = useState<Action | null>(null);
  const [message, setMessage] = useState("");
  const applied = async (result: LibraryApplied) => {
    if ("state" in result) {
      if (result.kind !== "import" || result.receipt.changed) {
        onIndexReplaced?.();
      }
      client.setQueryData(stateKey, result.state);
      await client.invalidateQueries({ queryKey: ["workbench", "capabilities"] });
    }
    switch (result.kind) {
      case "export":
        setMessage(`Saved copy: ${result.receipt.path}`);
        break;
      case "load":
        setMessage(`Loaded: ${result.state.library.path}`);
        break;
      case "new":
        setMessage(`Created: ${result.receipt.path}`);
        break;
      case "import":
        setMessage(
          result.receipt.changed
            ? `Imported: ${result.receipt.path}`
            : "Speaker Library is unchanged.",
        );
        break;
      default:
        unreachable(result);
    }
    setAction(null);
  };
  return (
    <section className="speakerIndex" aria-label="Speaker index">
      <div className="indexActions">
        <h2>Speaker index</h2>
        <button type="button" onClick={() => setAction("load")}>
          Load…
        </button>
        <button type="button" onClick={() => setAction("new")}>
          New…
        </button>
        <button
          type="button"
          disabled={state.data?.library.status !== "valid"}
          onClick={() => setAction("export")}
        >
          Save…
        </button>
        <button
          type="button"
          disabled={state.data?.library.status === "invalid"}
          onClick={() => setAction("import")}
        >
          Import…
        </button>
        {manageHref ? <a href={manageHref}>Manage speakers</a> : null}
      </div>
      {state.data ? (
        <>
          <p>
            {state.data.scope} · {state.data.library.profileCount}{" "}
            {state.data.library.profileCount === 1 ? "profile" : "profiles"} ·{" "}
            {state.data.library.status}
          </p>
          <code className="indexPath">{state.data.library.path}</code>
          {state.data.library.message ? <p>{state.data.library.message}</p> : null}
        </>
      ) : (
        <p>
          {state.isError
            ? "Could not read the active index. Load a valid directory or create one."
            : "Loading Speaker Directory…"}
        </p>
      )}
      {message ? <p role="status">{message}</p> : null}
      {action ? (
        <LibraryDialog action={action} onApplied={applied} onDismiss={() => setAction(null)} />
      ) : null}
    </section>
  );
}

function DialogFrame({
  title,
  onCancel,
  children,
}: {
  title: string;
  onCancel: () => void;
  children: ReactNode;
}) {
  const ref = useRef<HTMLDialogElement>(null);
  const id = useId();
  useEffect(() => {
    const dialog = ref.current;
    const previous = document.activeElement;
    dialog?.showModal();
    return () => {
      if (dialog?.open) {
        dialog.close();
      }
      if (previous instanceof HTMLElement) {
        previous.focus();
      }
    };
  }, []);
  return (
    <dialog
      className="libraryDialog"
      ref={ref}
      aria-labelledby={id}
      onCancel={(event) => {
        event.preventDefault();
        onCancel();
      }}
    >
      <h2 id={id}>{title}</h2>
      {children}
    </dialog>
  );
}

function LibraryDialog({
  action,
  onApplied,
  onDismiss,
}: {
  action: Action;
  onApplied: (result: LibraryApplied) => Promise<void>;
  onDismiss: () => void;
}) {
  const [scope, setScope] = useState<DirectoryChoice["scope"]>("explicit");
  const [path, setPath] = useState("");
  const [mode, setMode] = useState<"merge" | "replace">("merge");
  const [consent, setConsent] = useState(false);
  const [browsing, setBrowsing] = useState(false);
  const preview = useMutation({ mutationFn: speakerLibraryApi.preview });
  const confirm = useMutation({ mutationFn: speakerLibraryApi.confirm, onSuccess: onApplied });
  const load = useMutation({ mutationFn: speakerLibraryApi.load, onSuccess: onApplied });
  const cancel = useMutation({ mutationFn: speakerLibraryApi.cancel });
  const applying = confirm.isPending || load.isPending;
  const busy = applying || preview.isPending;
  const resetPreview = () => {
    if (preview.data) {
      cancel.mutate(preview.data.id);
    }
    preview.reset();
    confirm.reset();
    setConsent(false);
  };
  const dismiss = () => {
    if (applying) {
      return;
    }
    if (preview.data) {
      cancel.mutate(preview.data.id);
    }
    onDismiss();
  };
  const selection: DirectoryChoice = scope === "explicit" ? { scope, path } : { scope };
  const submit = () => {
    if (action === "load") {
      load.mutate(selection);
      return;
    }
    let request: LibraryPreviewRequest;
    switch (action) {
      case "new":
        request = { kind: "new", selection };
        break;
      case "export":
        request = { kind: "export", destination: path };
        break;
      case "import":
        request = { kind: "import", source: path, mode };
        break;
    }
    preview.mutate(request);
  };
  const error = preview.error ?? confirm.error ?? load.error ?? cancel.error;
  const unchanged = preview.data?.kind === "import" && !preview.data.summary.changed;

  return (
    <DialogFrame title={titles[action]} onCancel={dismiss}>
      <form
        onSubmit={(event) => {
          event.preventDefault();
          event.stopPropagation();
          submit();
        }}
      >
        <fieldset disabled={busy} onChange={resetPreview}>
          {action === "load" || action === "new" ? (
            <>
              <label>
                Directory scope
                <select
                  aria-label="Directory scope"
                  value={scope}
                  onChange={(event) => {
                    const value = event.currentTarget.value;
                    if (
                      value === "explicit" ||
                      value === "auto" ||
                      value === "local" ||
                      value === "global"
                    ) {
                      setScope(value);
                    }
                  }}
                >
                  <option value="explicit">Choose a directory</option>
                  <option value="auto">Automatic (local first)</option>
                  <option value="local">Project local</option>
                  <option value="global">Global</option>
                </select>
              </label>
              {scope === "explicit" ? (
                <label>
                  {action === "load" ? "Directory or library.json" : "Directory"}
                  <input
                    required
                    autoFocus
                    value={path}
                    onChange={(event) => setPath(event.currentTarget.value)}
                  />
                </label>
              ) : null}
            </>
          ) : (
            <label>
              {action === "export" ? "Destination file" : "Source file"}
              <input
                required
                autoFocus
                value={path}
                onChange={(event) => setPath(event.currentTarget.value)}
              />
            </label>
          )}
          {(action !== "load" && action !== "new") || scope === "explicit" ? (
            <>
              <button type="button" onClick={() => setBrowsing(!browsing)}>
                Browse…
              </button>
              {browsing ? (
                <LibraryPathPicker
                  kind={action === "import" ? "file" : "directory"}
                  onChoose={(chosen) => {
                    resetPreview();
                    setPath(action === "export" ? `${chosen}/speaker-library.json` : chosen);
                    setBrowsing(false);
                  }}
                />
              ) : null}
            </>
          ) : null}
          {action === "import" ? (
            <label>
              Import mode
              <select
                aria-label="Import mode"
                value={mode}
                onChange={(event) => {
                  const value = event.currentTarget.value;
                  if (value === "merge" || value === "replace") {
                    setMode(value);
                  }
                }}
              >
                <option value="merge">Merge compatible new profiles</option>
                <option value="replace">Replace the complete library</option>
              </select>
            </label>
          ) : null}
        </fieldset>
        <p className="muted">
          Explicit paths stay inside the launched directory. Saved copies use a new filename.
        </p>
        {error ? <p role="alert">{error.message}</p> : null}
        <button type="submit" disabled={busy}>
          {action === "load" ? "Load index" : "Review"}
        </button>
      </form>
      {preview.data ? (
        <div className="libraryConsent">
          <PreviewDetails preview={preview.data} />
          {unchanged ? (
            <p>The library already contains this exact snapshot. No files will change.</p>
          ) : (
            <>
              <label>
                <input
                  type="checkbox"
                  checked={consent}
                  disabled={applying}
                  onChange={(event) => setConsent(event.currentTarget.checked)}
                />
                {consentText(preview.data)}
              </label>
              <button
                type="button"
                disabled={!consent || applying}
                onClick={() => {
                  if (preview.data) {
                    confirm.mutate(preview.data.id);
                  }
                }}
              >
                {confirmationLabels[preview.data.kind]}
              </button>
            </>
          )}
        </div>
      ) : null}
      <button type="button" disabled={applying} onClick={dismiss}>
        {unchanged ? "Close" : "Cancel"}
      </button>
      {busy ? (
        <p role="status">
          {applying
            ? "Applying the selected operation…"
            : "Reading the library and checking the selection…"}
        </p>
      ) : null}
    </DialogFrame>
  );
}

function consentText(preview: LibraryPreview): string {
  switch (preview.kind) {
    case "new":
      return "Create this empty Speaker Library";
    case "export":
      return preview.summary.profileCount === 0
        ? "Save this empty Speaker Library copy"
        : "I agree to save voice embeddings locally to this destination";
    case "import":
      return preview.summary.incomingProfiles === 0
        ? "Replace the active library with this empty snapshot"
        : "I agree to import voice embeddings into this Speaker Library";
    default:
      return unreachable(preview);
  }
}

function PreviewDetails({ preview }: { preview: LibraryPreview }) {
  switch (preview.kind) {
    case "new":
      return (
        <>
          <p>Create an empty canonical library:</p>
          <p className="indexPath">{preview.summary.destination}</p>
          <p>Existing libraries will not be overwritten.</p>
        </>
      );
    case "export":
      return (
        <>
          <p>
            {preview.summary.profileCount} profiles · {preview.summary.confirmedProfiles} confirmed
            · {preview.summary.draftProfiles} drafts
            {preview.summary.unclassifiedProfiles > 0
              ? ` · ${preview.summary.unclassifiedProfiles} other statuses`
              : ""}
          </p>
          <p className="indexPath">{preview.summary.source}</p>
          <p>
            {preview.summary.profileCount === 0
              ? "Save this empty portable library to:"
              : "Save a portable copy containing voice embeddings to:"}
          </p>
          <p className="indexPath">{preview.summary.destination}</p>
        </>
      );
    case "import":
      return (
        <>
          <p>
            {preview.summary.incomingProfiles} incoming profiles ·{" "}
            {preview.summary.confirmedProfiles} confirmed · {preview.summary.draftProfiles} drafts
            {preview.summary.unclassifiedProfiles > 0
              ? ` · ${preview.summary.unclassifiedProfiles} other statuses`
              : ""}
          </p>
          <p className="indexPath">{preview.summary.source}</p>
          <p>
            {preview.summary.addedProfiles} added · {preview.summary.replacedProfiles} replaced ·{" "}
            {preview.summary.removedProfiles} removed · {preview.summary.unchangedProfiles}{" "}
            unchanged
          </p>
          <p>Destination:</p>
          <p className="indexPath">{preview.summary.destination}</p>
          {preview.summary.backupPath ? (
            <>
              <p>Previous library backup:</p>
              <p className="indexPath">{preview.summary.backupPath}</p>
            </>
          ) : null}
        </>
      );
    default:
      return unreachable(preview);
  }
}
