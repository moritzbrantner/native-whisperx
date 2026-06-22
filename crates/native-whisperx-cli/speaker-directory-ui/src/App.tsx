import { QueryClient, QueryClientProvider, useQuery } from "@tanstack/react-query";
import { useState } from "react";

import { speakerDirectoryApi, type SpeakerDirectoryState } from "./api";
import "./styles.css";

const queryClient = new QueryClient();

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <SpeakerDirectorySummary />
    </QueryClientProvider>
  );
}

function SpeakerDirectorySummary() {
  const stateQuery = useQuery({
    queryKey: ["speaker-directory-state"],
    queryFn: () => speakerDirectoryApi.getState(),
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

  return <SpeakerDirectoryStateView state={stateQuery.data} />;
}

function SpeakerDirectoryStateView({ state }: { state: SpeakerDirectoryState }) {
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
            <article className="profile" key={profile.id}>
              <h3>{profile.label}</h3>
              <p className="mono">{profile.id}</p>
              <dl>
                {Object.entries(profile.metadata).map(([key, value]) => (
                  <div key={key}>
                    <dt>{key}</dt>
                    <dd>{value}</dd>
                  </div>
                ))}
              </dl>
            </article>
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
