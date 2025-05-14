import franken from "franken-ui/shadcn-ui/preset-quick";

/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  presets: [franken()],
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  safelist: [
    {
      pattern: /^uk-/,
    },
    "ProseMirror",
    "ProseMirror-focused",
    "tiptap",
    "mr-2",
    "mt-2",
    "opacity-50",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
