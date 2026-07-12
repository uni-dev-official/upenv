import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// Tauri expects a fixed port and will fail if it's occupied.
// https://v2.tauri.app/start/frontend/vite/
export default defineConfig(async () => ({
  plugins: [react(), tailwindcss()],

  // Vite options tailored for Tauri development
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: "0.0.0.0",
    watch: {
      // Tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
