import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

const realApiBase = process.env.NATIVE_WHISPERX_SPEAKER_DIRECTORY_API_BASE;

export default defineConfig({
  plugins: [react()],
  define: {
    __NATIVE_WHISPERX_REAL_API_BASE_CONFIGURED__: JSON.stringify(Boolean(realApiBase)),
  },
  ...(realApiBase
    ? { server: { proxy: { "/api": { target: realApiBase, changeOrigin: true } } } }
    : {}),
  test: {
    environment: "jsdom",
    setupFiles: "./vitest.setup.ts",
  },
});
