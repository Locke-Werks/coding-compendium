import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// Tauri serves the frontend from a fixed port in development and from bundled
// files in release. The settings below are the ones Tauri needs; the rest are
// Vite defaults.
export default defineConfig({
  plugins: [react(), tailwindcss()],

  // Tauri launches before Vite is ready and expects this exact port, so failing
  // loudly beats silently moving to 1421 and leaving the window blank.
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // Rust rebuilds are driven by cargo, and watching target/ means thousands
      // of files churning on every build.
      ignored: ["**/src-tauri/**", "**/build/**"],
    },
  },

  // Windows 11 ships WebView2, which is current Chromium, so there is no reason
  // to down-compile.
  build: {
    target: "chrome120",
    // Source maps only in development: they roughly double the bundle and the
    // release build ships to one person who will not be opening devtools.
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
    minify: process.env.TAURI_ENV_DEBUG ? false : "esbuild",
  },
});
