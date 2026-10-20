// Optional real-browser smoke. Model calls are protocol-faithful boundary doubles;
// audio decoding, speaker preview, workflow, projections and the UI are real.
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";

const { chromium } = await import(process.env.PLAYWRIGHT_MODULE ?? "playwright");
const base = process.env.SITE_URL;
assert(base, "Set SITE_URL to the locally served prepared site.");
const artifacts = process.env.SITE_EVIDENCE_DIR;
if (artifacts) await fs.mkdir(artifacts, { recursive: true });
const browser = await chromium.launch({
  headless: true,
  ...(process.env.CHROMIUM_PATH ? { executablePath: process.env.CHROMIUM_PATH } : {}),
});
const errors = [];
const forbidden = [];
const origin = new URL(base).origin;

// A small generated WAV exercises real browser decode and speaker preview.
const wav = Buffer.alloc(44 + 32000 * 2);
wav.write("RIFF");
wav.writeUInt32LE(wav.length - 8, 4);
wav.write("WAVEfmt ", 8);
wav.writeUInt32LE(16, 16);
wav.writeUInt16LE(1, 20);
wav.writeUInt16LE(1, 22);
wav.writeUInt32LE(16000, 24);
wav.writeUInt32LE(32000, 28);
wav.writeUInt16LE(2, 32);
wav.writeUInt16LE(16, 34);
wav.write("data", 36);
wav.writeUInt32LE(wav.length - 44, 40);
for (let i = 0; i < 32000; i++)
  wav.writeInt16LE(
    Math.round(9000 * Math.sin((2 * Math.PI * (i < 16000 ? 180 : 820) * i) / 16000)),
    44 + i * 2,
  );

async function open({
  mobile = false,
  supported = true,
  translation = true,
  acceptance = false,
} = {}) {
  const page = await browser.newPage({
    viewport: mobile ? { width: 390, height: 844 } : { width: 1280, height: 900 },
    ...(mobile ? { hasTouch: true, isMobile: true } : {}),
  });
  page.on("pageerror", (error) => errors.push(error.message));
  await page.route("**/*", async (route) => {
    const request = route.request();
    const url = new URL(request.url());
    if (url.protocol === "blob:" || url.protocol === "data:") return route.continue();
    if (url.origin !== origin || !["GET", "HEAD"].includes(request.method())) {
      forbidden.push(`${request.method()} ${url.origin}${url.pathname}`);
      return route.abort();
    }
    if (url.pathname.endsWith("/vendor/audio-analysis-transcription.js")) {
      return route.fulfill({
        contentType: "text/javascript",
        body: `
        export const browserTranscriptionLanguages = () => ["en", "de", "ja", "fr"];
        export const browserTranscriptionModels = () => [{id:"fixture",label:"Fixture Whisper",description:"Deterministic model boundary"}];
        export const browserTranscriptionCapabilities = () => ({runtime:"fixture-webgpu-boundary",modelId:"fixture"});
        export const supportsBrowserTranscription = async () => ${supported};
        export async function transcribeAudioSamples(samples, options) {
          if (!(samples instanceof Float32Array) || samples.length !== 32000) throw Error("Expected decoded 16 kHz PCM");
          globalThis.fixtureCalls = [...(globalThis.fixtureCalls ?? []), "transcribe"];
          globalThis.fixtureLanguages = [...(globalThis.fixtureLanguages ?? []), options.language ?? null];
          options.onProgress({stage:"transcribe",message:"Transcribing fixture audio…"});
          await new Promise(resolve => setTimeout(resolve, 250));
          return {text:"Hello world.",language:"en",segments:[
            {index:7,text:"Hello",startSeconds:0,endSeconds:1},
            {index:9,text:"world.",startSeconds:1,endSeconds:2}
          ],attributes:{}};
        }
      `,
      });
    }
    if (url.pathname.endsWith("/vendor/audio-analysis-alignment.js")) {
      return route.fulfill({contentType:"text/javascript", body:`
        export const browserAlignmentCapabilities = () => ({languages:["en","de"]});
        export const supportsBrowserAlignment = async () => true;
        export async function alignTranscriptAudioSamples(samples, source, options) {
          if (!(samples instanceof Float32Array) || samples.length !== 32000 || options.language !== "en") throw Error("Incorrect alignment input");
          globalThis.fixtureCalls = [...(globalThis.fixtureCalls ?? []), "align"];
          await new Promise(resolve => setTimeout(resolve, 250));
          options.signal.throwIfAborted();
          if (globalThis.fixtureAlignmentFailure) throw Error("Fixture alignment failure");
          return {...source, segments:source.segments.map(segment=>({...segment,words:[{text:segment.text,startSeconds:segment.startSeconds,endSeconds:segment.endSeconds}]})),attributes:{alignment:"ctc-forced-alignment"}};
        }
      `});
    }
    if (url.pathname.endsWith("/vendor/browser-translation.js")) {
      return route.fulfill({
        contentType: "text/javascript",
        body: `
        export const browserTranslationCapabilities = () => ({runtime:"fixture-translation-boundary",pairs:[
          {sourceLanguage:"de",targetLanguage:"en",modelId:"fixture-de-en"},
          {sourceLanguage:"en",targetLanguage:"de",modelId:"fixture-en-de"},
          {sourceLanguage:"fr",targetLanguage:"en",modelId:"fixture-fr-en"}
        ]});
        export const supportsBrowserTranslation = async () => ${translation};
        export async function translateBrowserSegments(segments, options) {
          globalThis.fixtureCalls = [...(globalThis.fixtureCalls ?? []), "translate"];
          globalThis.fixtureTranslationDirections = [...(globalThis.fixtureTranslationDirections ?? []), options.sourceLanguage+"->"+options.targetLanguage];
          options.onProgress({stage:"translate",message:"Translating fixture segments…"});
          return {segments:segments.map(segment=>({...segment,text:segment.id===7?"Hallo":"Welt."})),
            sourceLanguage:options.sourceLanguage,targetLanguage:options.targetLanguage,attributes:{fixture:true}};
        }
      `,
      });
    }
    return route.continue();
  });
  await page.goto(`${base}/${acceptance ? "acceptance/" : "workbench.html"}`);
  return page;
}
async function selectFile(page) {
  await page
    .getByLabel("Audio file", { exact: true })
    .setInputFiles({ name: "fixture.wav", mimeType: "audio/wav", buffer: wav });
}
async function completed(page) {
  await page
    .locator("#browser-status")
    .filter({ hasText: /^Finished locally/ })
    .waitFor();
}
try {
  const page = await open();
  await page.getByText("WebGPU ready", { exact: true }).waitFor();
  assert.equal(await page.locator(".browser-advanced").getAttribute("open"), null);
  assert.equal(await page.locator("#native-workflow").getAttribute("open"), null);
  await selectFile(page);
  assert.equal(await page.locator("#browser-language").count(), 1);
  await page.getByLabel("Transcription language", { exact: true }).selectOption("en");
  await page.getByRole("checkbox", { name: /Anonymous speaker labels/ }).check();
  await page.getByRole("checkbox", { name: /^Translate/ }).check();
  await page.getByLabel("Translation pair", { exact: true }).selectOption("en-de");
  await page.getByRole("checkbox", {name:/Align timestamps/}).check();
  const run = page.locator("#run-browser");
  const box = await run.boundingBox();
  assert(
    box && box.y + box.height < 900,
    `Run must fit above desktop fold after choosing tasks and input: ${JSON.stringify(box)}`,
  );
  await run.focus();
  await page.keyboard.press("Enter");
  await completed(page);
  assert.deepEqual(await page.evaluate(() => globalThis.fixtureCalls), ["transcribe", "align", "translate"]);
  assert.deepEqual(await page.evaluate(() => globalThis.fixtureLanguages), ["en"]);
  assert.equal(await page.locator("#source-transcript-text").textContent(), "Hello world.");
  assert(await page.locator("#transcript").isHidden());
  assert.match(await page.locator("#segment-rows").textContent(), /Hallo/);
  assert.match(await page.locator("#segment-rows tr").first().textContent(), /00:00:00\.000 – 00:00:01\.000/);
  assert.equal(await page.locator("#segment-rows tr").count(), 2);
  assert.equal(await page.evaluate(() => document.documentElement.dataset.audioDecodeCount), "1");
  for (const format of ["Native JSON", "TXT", "SRT", "WebVTT"]) {
    const [download] = await Promise.all([
      page.waitForEvent("download"),
      page.getByRole("button", { name: `Download ${format}`, exact: true }).click(),
    ]);
    const content = await fs.readFile(await download.path(), "utf8");
    assert.match(content, /Hallo/);
    if (format === "Native JSON") {
      const contract = JSON.parse(content);
      assert.deepEqual(
        contract.segments.map((s) => [s.index, s.startSeconds, s.endSeconds]),
        [
          [7, 0, 1],
          [9, 1, 2],
        ],
      );
      assert.equal(contract.attributes.alignment, "ctc-forced-alignment");
      assert(contract.segments.every(segment => segment.words.length === 0));
      assert.equal(await page.evaluate(() => document.documentElement.dataset.alignmentCompleted), "true");
      assert.equal(contract.attributes.diarization, "completed");
      assert.equal(contract.language, "de");
      assert(contract.segments.every((s) => s.speaker?.startsWith("speaker_")));
    }
  }
  if (artifacts)
    await page.screenshot({ path: path.join(artifacts, "desktop.png"), fullPage: true });
  await page.getByRole("checkbox", {name:/^Translate/}).uncheck();
  await run.click();
  await completed(page);
  const [alignedDownload] = await Promise.all([
    page.waitForEvent("download"), page.getByRole("button", {name:"Download Native JSON", exact:true}).click(),
  ]);
  const alignedContract = JSON.parse(await fs.readFile(await alignedDownload.path(), "utf8"));
  assert(alignedContract.segments.every(segment => segment.words.length === 1));
  assert(alignedContract.segments.every(segment => segment.speaker?.startsWith("speaker_")));
  await page.getByRole("checkbox", {name:/^Translate/}).check();
  await page.evaluate(() => {
    globalThis.fixtureCalls = [];
  });
  await run.click();
  await page.waitForFunction(() => globalThis.fixtureCalls?.length === 1);
  assert(await page.getByRole("button", { name: "Choose another file", exact: true }).isDisabled());
  assert(await page.getByRole("checkbox", { name: /Anonymous speaker labels/ }).isDisabled());
  await page.locator("#cancel-browser").click();
  await page
    .locator("#browser-status")
    .filter({ hasText: /^Browser workflow cancelled/ })
    .waitFor();
  assert.deepEqual(await page.evaluate(() => globalThis.fixtureCalls), ["transcribe"]);
  assert(await page.locator("#download-actions").isHidden());
  await run.click();
  await completed(page);
  await page.evaluate(() => { globalThis.fixtureCalls = []; });
  await run.click();
  await page.waitForFunction(() => globalThis.fixtureCalls?.includes("align"));
  await page.locator("#cancel-browser").click();
  await page.locator("#browser-status").filter({hasText:/^Browser workflow cancelled/}).waitFor();
  assert.deepEqual(await page.evaluate(() => globalThis.fixtureCalls), ["transcribe", "align"]);
  assert(await page.locator("#download-actions").isHidden());
  await page.evaluate(() => { globalThis.fixtureAlignmentFailure = true; });
  await run.click();
  await page.locator("#browser-status").filter({hasText:/Fixture alignment failure/}).waitFor();
  assert(await page.locator("#download-actions").isHidden());
  await page.evaluate(() => { globalThis.fixtureAlignmentFailure = false; });
  await run.click();
  await completed(page);
  const mobile = await open({ mobile: true });
  await mobile.getByText("WebGPU ready", { exact: true }).waitFor();
  await selectFile(mobile);
  await mobile.getByRole("checkbox", { name: /Anonymous speaker labels/ }).tap();
  await mobile.getByRole("checkbox", { name: /^Translate/ }).tap();
  await mobile.getByLabel("Translation pair", { exact: true }).selectOption("en-de");
  await mobile.getByRole("checkbox", {name:/Align timestamps/}).tap();
  await mobile.locator("#run-browser").tap();
  await completed(mobile);
  assert.equal(await mobile.evaluate(() => document.documentElement.scrollWidth), 390);
  await mobile.evaluate(() => window.scrollTo(0, 0));
  if (artifacts)
    await mobile.screenshot({ path: path.join(artifacts, "mobile.png"), fullPage: true });
  const languages = await open();
  await languages.getByText("WebGPU ready", {exact:true}).waitFor();
  await selectFile(languages);
  const language = languages.getByLabel("Transcription language", {exact:true});
  await language.selectOption("ja");
  await languages.getByRole("checkbox", {name:/Align timestamps/}).check();
  assert(await languages.locator("#run-browser").isDisabled());
  await languages.getByRole("checkbox", {name:/Align timestamps/}).uncheck();
  await languages.locator("#run-browser").click();
  await completed(languages);
  assert.deepEqual(await languages.evaluate(() => globalThis.fixtureLanguages), ["ja"]);
  await language.selectOption("");
  await languages.locator("#run-browser").click();
  await completed(languages);
  assert.deepEqual(await languages.evaluate(() => globalThis.fixtureLanguages), ["ja", null]);
  await language.selectOption("ja");
  await languages.getByRole("checkbox", {name:/^Translate/}).check();
  assert(await languages.locator("#run-browser").isDisabled());
  assert(await languages.locator("#browser-language-problem").isVisible());
  await languages.getByLabel("Translation pair", {exact:true}).selectOption("en-de");
  assert.equal(await language.inputValue(), "en");
  assert(await languages.locator("#run-browser").isEnabled());
  await languages.getByLabel("Translation pair", {exact:true}).selectOption("fr-en");
  assert.equal(await language.inputValue(), "fr");
  await languages.locator("#run-browser").click();
  await completed(languages);
  assert.deepEqual(await languages.evaluate(() => globalThis.fixtureTranslationDirections), ["fr->en"]);
  const absent = await open({ supported: false, translation: false });
  await absent.getByText("WebGPU unavailable", { exact: true }).waitFor();
  await selectFile(absent);
  assert(await absent.locator("#run-browser").isDisabled());
  assert(await absent.getByRole("checkbox", { name: /^Translate/ }).isDisabled());
  assert(await absent.getByRole("checkbox", { name: /Align timestamps/ }).isEnabled());
  await absent.locator("#native-workflow > summary").click();
  assert.match(await absent.locator("#native-command").textContent(), /native-whisperx/);
  const partial = await open({ translation: false });
  await partial.getByText("WebGPU ready", { exact: true }).waitFor();
  await selectFile(partial);
  assert(await partial.getByRole("checkbox", { name: /^Translate/ }).isDisabled());
  assert(await partial.locator("#run-browser").isEnabled());
  const harness = await open({ acceptance: true });
  const frame = harness.frameLocator("#workbench");
  await frame.getByText("WebGPU ready", { exact: true }).waitFor();
  await frame
    .getByLabel("Audio file", { exact: true })
    .setInputFiles({ name: "fixture.wav", mimeType: "audio/wav", buffer: wav });
  await frame.locator("#run-browser").click();
  await frame
    .locator("#browser-status")
    .filter({ hasText: /^Finished locally/ })
    .waitFor();
  await harness.locator("#capture").click();
  const report = JSON.parse(await harness.locator("#report").textContent());
  assert.equal(report.checks.decodedExactlyOnce, true);
  assert.equal(report.checks.projectionsAvailable, true);
  assert.equal(report.segmentCount, 2);
  await page.goto(`${base}/transcribe/`);
  await page.waitForURL(/workbench.html#browser-preview/);
  assert.deepEqual(errors, []);
  assert.deepEqual(forbidden, []);
  const observations = {
    fixtureModelBoundaries: true,
    physicalWebGpuVerified: false,
    realDecodeAndSpeakerPreview: true,
    combinedStages: true,
    downloads: 4,
    cancelledBeforeLaterStages: true,
    retry: true,
    desktopRunAboveFold: true,
    touchWidth: 390,
    noWebGpuDisabled: true,
    translationUnavailableStillAllowsAsr: true,
    explicitAndAutomaticLanguages: true,
    translationLanguageGuard: true,
    compatibilityRoute: true,
    acceptanceHarnessUnchanged: true,
    errors,
    forbidden,
  };
  if (artifacts)
    await fs.writeFile(
      path.join(artifacts, "observations.json"),
      JSON.stringify(observations, null, 2) + "\n",
    );
  console.log(JSON.stringify(observations));
} finally {
  await browser.close();
}
