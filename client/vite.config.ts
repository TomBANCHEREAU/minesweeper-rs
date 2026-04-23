import preact from "@preact/preset-vite";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [preact()],
  dev: {},

  server: {
    proxy: {
      "/api": { target: "http://127.0.0.1:9000", ws: true },
    },
  },
});
