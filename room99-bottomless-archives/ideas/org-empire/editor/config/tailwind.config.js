const TypographyPlugin = require("@tailwindcss/typography");
const FormsPlugin = require("@tailwindcss/forms");
const AspectRatioPlugin = require("@tailwindcss/aspect-ratio");
const DaisyUiPlugin = require("daisyui");

const IS_PROD = process.env.NODE_ENV === "production";
const JIT = process.env.JIT === "true" || !IS_PROD;

module.exports = {
  mode: JIT && "jit",
  content: ["src/**/*"],
  darkMode: "class", // false or 'media' or 'class',
  plugins: [
    TypographyPlugin,
    FormsPlugin,
    AspectRatioPlugin,
    DaisyUiPlugin,
  ],
};
