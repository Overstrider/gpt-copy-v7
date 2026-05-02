import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./src/**/*.{ts,tsx}", "./e2e/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        canvas: "#f7f7f5",
        ink: "#161616",
        muted: "#6f6f6f",
        panel: "#ffffff",
        line: "#deded8",
        accent: "#0f766e",
        accentSoft: "#ccfbf1",
        warning: "#b45309"
      },
      boxShadow: {
        soft: "0 18px 45px rgba(23, 23, 20, 0.09)"
      }
    }
  },
  plugins: []
};

export default config;
