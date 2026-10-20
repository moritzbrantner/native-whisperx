# Additional browser translation languages

The website consumes platform-packages source
`67dc0a6331a0414025ea268731cfea962e6dee58` and exposes its actual capability catalog:
German, French, Spanish and Dutch in both directions with English (eight pairs).
The local app retains its existing eight-language native translation catalog.
Unavailable pairs are not offered; selecting a pair selects its required source
language, and an incompatible source choice still disables Run.

Site preparation and static checks pass. The existing desktop/mobile workflow
suite passes, with an additional French → English selection/forwarding check,
retained source segment identity/timing, cancellation, failure/retry and download
checks. Its model boundaries are fixtures; it does not claim physical GPU evidence.

Upstream [real-model evidence](https://github.com/moritzbrantner/platform-packages/blob/67dc0a6331a0414025ea268731cfea962e6dee58/packages/browser-translation/docs/language-models.md)
records successful execution of all six new directions through the public adapter
in Chromium with actual pinned Q4 models and SwiftShader software WebGPU. Those
short fixtures establish compatibility, not broad accuracy or physical GPU speed.
The actual model revision is forwarded to acquisition and returned in provenance.
No registry publication, version bump or alternate translation runtime is required.

Shared convention sourceRevision: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.
