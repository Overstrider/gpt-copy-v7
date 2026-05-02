import nextVitals from "eslint-config-next/core-web-vitals";
import testingLibrary from "eslint-plugin-testing-library";

const config = [
  ...nextVitals,
  {
    files: ["src/test/**/*.{ts,tsx}"],
    plugins: {
      "testing-library": testingLibrary
    },
    rules: {
      ...testingLibrary.configs.react.rules
    }
  },
  {
    ignores: [".next/**", "node_modules/**", "playwright-report/**", "test-results/**"]
  }
];

export default config;
