import tailwind from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "path";

export default {
  plugins: [react(), tailwind()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
  server: {
    port: 1420,
  },
};
