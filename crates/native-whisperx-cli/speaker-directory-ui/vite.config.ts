import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

const realApiBase = process.env.NATIVE_WHISPERX_SPEAKER_DIRECTORY_API_BASE;

export default defineConfig({
  plugins: [react()],
  define: {
    __NATIVE_WHISPERX_REAL_API_BASE_CONFIGURED__: JSON.stringify(Boolean(realApiBase)),
  },
  server: realApiBase
    ? {
        proxy: {
          "/api": {
            target: realApiBase,
            changeOrigin: true,
          },
        },
      }
    : undefined,
  test: {
    environment: "jsdom",
    setupFiles: "./vitest.setup.ts",
  },
});
