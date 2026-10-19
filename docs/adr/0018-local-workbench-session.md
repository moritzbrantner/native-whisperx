# ADR 0018: Adapt the finite workflow through a local workbench session

Status: accepted for #377.

The local workbench extends the existing CLI-served React Speaker Directory UI.
`workbench` opens the Transcribe destination; `speakers open` continues opening
Speakers. Navigation uses non-sensitive `view` query parameters. File paths,
model choices and job observations do not become browser navigation parameters.

The server adapts a fixed, typed product request into one
`NativeWhisperxConfig` and calls `run_with_control` once. Task selection cannot
supply executable stages or reorder dependencies. Authoritative finite workflow
validation, resource resolution, progress, cancellation and output placement
remain with the existing product engine and its upstream capabilities.

The launched working directory bounds file browsing, explicit resource paths,
input selection, output folders and result access. Paths are canonicalized and
must remain inside that directory, including through symlinks. Standard model
cache lookup remains owned by the existing workflow. Explicit output folders
must already exist; omitted output folders retain Input-Local Output. The
selected Speaker Directory remains the existing independently resolved identity
location, not a new speaker index or a copy of the trace.

One session-owned native worker executes at most one finite file at a time.
The HTTP loop serves state and cancellation while inference runs. Cancellation
uses the existing cooperative safe boundaries. Worker creation and execution
failures become errors; they never become completed output. Dropping the session
cancels and joins its worker. There is no general job executor or background
process invocation API.

Every workbench operation, including reads, requires a cryptographically random
session token. Token generation fails closed on unavailable system entropy.
The server validates its exact loopback Host, any Origin, and Fetch Metadata
before returning HTML, state or mutations. Cross-origin requests are rejected;
no CORS execution bridge is provided. Request size and read/write time are
bounded. The existing Speaker Directory operations retain their routes and
session-token mutation contract.

The app shows actual task states and model lifecycle observations, without
invented percentages. Starting a job acknowledges execution; it does not claim
model readiness or completion. Missing cache entries, disabled features and
provider errors remain visible failures. Model downloads remain a deliberate
setup step outside this cache-only UI workflow. Result access is restricted to
an emitted output of the completed session job and reads that file again; a
path supplied by the browser cannot choose an arbitrary result file.
