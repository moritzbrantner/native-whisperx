import { useMutation } from "@tanstack/react-query";

import { workbenchApi } from "./workbenchApi";

export function LibraryPathPicker({
  kind,
  onChoose,
}: {
  kind: "directory" | "file";
  onChoose: (path: string) => void;
}) {
  const files = useMutation({ mutationFn: workbenchApi.files });
  return (
    <div className="filePicker" aria-label="Local library paths">
      <button type="button" disabled={files.isPending} onClick={() => files.mutate(".")}>
        Browse launched directory
      </button>
      {files.error ? <p role="alert">{files.error.message}</p> : null}
      {files.data ? (
        <>
          <p className="indexPath">
            {files.data.root}/{files.data.directory}
          </p>
          <button
            type="button"
            disabled={!files.data.directory || files.isPending}
            onClick={() => files.mutate(`${files.data?.directory}/..`)}
          >
            Parent folder
          </button>
          {kind === "directory" ? (
            <button type="button" onClick={() => onChoose(files.data?.directory || ".")}>
              Use this folder
            </button>
          ) : null}
          <ul>
            {files.data.entries
              .filter(
                (entry) =>
                  entry.directory ||
                  (kind === "file" &&
                    (entry.name.endsWith(".json") || entry.name.endsWith(".backup"))),
              )
              .map((entry) => (
                <li key={entry.name}>
                  <button
                    type="button"
                    disabled={files.isPending}
                    onClick={() => {
                      if (entry.directory) {
                        files.mutate(entry.path);
                      } else {
                        onChoose(entry.path);
                      }
                    }}
                  >
                    {entry.name}
                    {entry.directory ? "/" : ""}
                  </button>
                </li>
              ))}
          </ul>
          {files.data.truncated ? (
            <p>More files exist. Enter an exact path in the field above.</p>
          ) : null}
        </>
      ) : null}
    </div>
  );
}
