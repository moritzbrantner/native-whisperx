import {
  QueryClient,
  QueryClientProvider,
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { useState } from "react";

import {
  speakerDirectoryApi,
  type SpeakerDirectoryApi,
  type SpeakerDirectoryState,
  type SpeakerProfileState,
} from "./api";
import "./styles.css";

export function App({ api = speakerDirectoryApi }: { api?: SpeakerDirectoryApi }) {
  const [queryClient] = useState(() => new QueryClient());
  return (
    <QueryClientProvider client={queryClient}>
      <SpeakerDirectorySummary api={api} />
    </QueryClientProvider>
  );
}

function SpeakerDirectorySummary({ api }: { api: SpeakerDirectoryApi }) {
  const stateQuery = useQuery({
    queryKey: ["speaker-directory-state"],
    queryFn: () => api.getState(),
  });

  if (stateQuery.isLoading) {
    return <main className="page">Loading Speaker Directory...</main>;
  }

  if (stateQuery.isError || !stateQuery.data) {
    return (
      <main className="page">
        <h1>Speaker Directory</h1>
        <p role="alert">Failed to load Speaker Directory state.</p>
      </main>
    );
  }

  return <SpeakerDirectoryStateView api={api} state={stateQuery.data} />;
}

function SpeakerDirectoryStateView({
  api,
  state,
}: {
  api: SpeakerDirectoryApi;
  state: SpeakerDirectoryState;
}) {
  const [scanRoot] = useState(state.trace.scanRoot ?? "");
  const anonymousSpeakers = state.trace.speakers.filter((speaker) => speaker.kind === "anonymous");

  return (
    <main className="page">
      <header className="header">
        <p className="eyebrow">CLI workspace</p>
        <h1>Speaker Directory</h1>
        <p className="path">{state.path}</p>
      </header>

      <section className="summaryGrid" aria-label="Speaker Directory summary">
        <StatusPanel
          title="Speaker Library"
          status={state.library.status}
          detail={`${state.library.profileCount} profile${
            state.library.profileCount === 1 ? "" : "s"
          }`}
        />
        <StatusPanel
          title="Speaker Trace"
          status={state.trace.status}
          detail={state.trace.scanRoot ?? "No scan root"}
        />
        <StatusPanel title="Scope" status={state.scope} detail="Speaker Directory" />
      </section>

      <section>
        <div className="sectionHeading">
          <h2>Speaker Library profiles</h2>
          <span>{state.profiles.length}</span>
        </div>
        <div className="profileList">
          {state.profiles.map((profile) => (
            <SpeakerLibraryProfileCard api={api} key={profile.id} profile={profile} />
          ))}
        </div>
      </section>

      <section>
        <div className="sectionHeading">
          <h2>Speaker Trace</h2>
          <span>{state.trace.speakers.length}</span>
        </div>
        <div className="traceMeta">
          <span>Scan root</span>
          <code>{scanRoot || "Not available"}</code>
        </div>
        <div className="profileList">
          {state.trace.speakers.map((speaker) => (
            <article className="profile" key={speaker.profileId ?? speaker.anonymousLabel}>
              <h3>{speaker.label ?? speaker.anonymousLabel ?? "Anonymous Speaker Label"}</h3>
              <p className="mono">
                {speaker.kind === "anonymous" ? "Anonymous Speaker Label" : speaker.profileId}
              </p>
              <p>{speaker.files.length} traced file(s)</p>
            </article>
          ))}
        </div>
      </section>

      <section>
        <div className="sectionHeading">
          <h2>Anonymous Speaker Label</h2>
          <span>{anonymousSpeakers.length}</span>
        </div>
        <p className="muted">
          Anonymous Speaker Labels are Speaker Trace data, not enrolled Speaker Library identities.
        </p>
      </section>
    </main>
  );
}

function StatusPanel({ title, status, detail }: { title: string; status: string; detail: string }) {
  return (
    <article className="statusPanel">
      <h2>{title}</h2>
      <p className="status">{status}</p>
      <p>{detail}</p>
    </article>
  );
}

function SpeakerLibraryProfileCard({
  api,
  profile,
}: {
  api: SpeakerDirectoryApi;
  profile: SpeakerProfileState;
}) {
  const queryClient = useQueryClient();
  const [label, setLabel] = useState(profile.label);
  const [metadataText, setMetadataText] = useState(formatMetadata(profile.metadata));
  const [formError, setFormError] = useState<string | null>(null);

  const updateProfile = useMutation({
    mutationFn: () => {
      const metadata = parseMetadata(metadataText);
      setFormError(null);
      return api.updateProfile(profile.id, { id: profile.id, label, metadata });
    },
    onSuccess: (state) => {
      queryClient.setQueryData(["speaker-directory-state"], state);
    },
    onError: (error) => {
      setFormError(
        error instanceof Error ? error.message : "Failed to save Speaker Library profile.",
      );
    },
  });

  const deleteProfile = useMutation({
    mutationFn: () => {
      setFormError(null);
      return api.deleteProfile(profile.id);
    },
    onSuccess: (state) => {
      queryClient.setQueryData(["speaker-directory-state"], state);
    },
    onError: (error) => {
      setFormError(
        error instanceof Error ? error.message : "Failed to delete Speaker Library profile.",
      );
    },
  });

  const saveProfile = () => {
    try {
      parseMetadata(metadataText);
    } catch (error) {
      setFormError(
        error instanceof Error ? error.message : "Speaker Library profile metadata is malformed.",
      );
      return;
    }
    updateProfile.mutate();
  };

  return (
    <article className="profile">
      <div className="profileIdentity">
        <div>
          <h3>{profile.label}</h3>
          <p className="identityLabel">Stable profile id</p>
          <p className="mono profileId">{profile.id}</p>
        </div>
        <span className="identityBadge">Speaker Library profile</span>
      </div>
      <dl>
        {Object.entries(profile.metadata).map(([key, value]) => (
          <div key={key}>
            <dt>{key}</dt>
            <dd>{value}</dd>
          </div>
        ))}
      </dl>
      <div className="profileForm">
        <label>
          Label
          <input
            aria-label={`${profile.id} label`}
            value={label}
            onChange={(event) => setLabel(event.currentTarget.value)}
          />
        </label>
        <label>
          Metadata
          <textarea
            aria-label={`${profile.id} metadata`}
            rows={4}
            value={metadataText}
            onChange={(event) => setMetadataText(event.currentTarget.value)}
          />
        </label>
        {formError ? <p role="alert">{formError}</p> : null}
        <div className="profileActions">
          <button disabled={updateProfile.isPending} type="button" onClick={saveProfile}>
            Save profile
          </button>
          <button
            disabled={deleteProfile.isPending}
            type="button"
            onClick={() => deleteProfile.mutate()}
          >
            Delete profile
          </button>
        </div>
      </div>
    </article>
  );
}

function formatMetadata(metadata: Record<string, string>) {
  return Object.entries(metadata)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");
}

function parseMetadata(text: string): Record<string, string> {
  if (!text) {
    return {};
  }
  return text.split("\n").reduce<Record<string, string>>((metadata, line, index) => {
    if (!line.trim()) {
      throw new Error(`Speaker Library profile metadata line ${index + 1} must be key=value.`);
    }
    const separator = line.indexOf("=");
    if (separator <= 0 || separator === line.length - 1) {
      throw new Error(`Speaker Library profile metadata line ${index + 1} must be key=value.`);
    }
    const key = line.slice(0, separator).trim();
    const value = line.slice(separator + 1).trim();
    if (!key || !value) {
      throw new Error(`Speaker Library profile metadata line ${index + 1} must be key=value.`);
    }
    metadata[key] = value;
    return metadata;
  }, {});
}
