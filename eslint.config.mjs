import { defineConfig } from "eslint-define-config";

export default defineConfig({
  // … ton config existante …
  plugins: ["unicorn", /* … */],
  extends: [
    "plugin:unicorn/recommended",
    // …
  ],
  rules: {
    // tes règles globales
  },
  overrides: [
    {
      // Tests et specs
      files: ["test/**/*.ts", "anchor/tests/**/*.ts"],
      rules: {
        "unicorn/filename-case": "off",
        "unicorn/catch-error-name": "off",
        "unicorn/prefer-type-error": "off",
        "unicorn/no-null": "off",
      },
    },
    {
      // Scripts de migration / CLI
      files: ["bin/**/*.js", "anchor/migrations/**/*.ts"],
      rules: {
        "unicorn/filename-case": "off",
        "unicorn/no-anonymous-default-export": "off",
        "unicorn/numeric-separators-style": "off",
      },
    },
    {
      // Composants React front
      files: ["src/components/**/*.js", "src/components/**/*.tsx"],
      rules: {
        "unicorn/no-null": "off",
        "unicorn/prefer-logical-operator-over-ternary": "off",
        "unicorn/prefer-number-properties": "off",
      },
    },
  ],
});
