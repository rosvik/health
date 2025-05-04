import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    port: 3000,
    proxy: {
      "/api/health": {
        target: "http://127.0.0.1:8603",
      },
    },
  },
  build: {
    target: "esnext",
    outDir: "../dist",
    emptyOutDir: true,
  },
});
