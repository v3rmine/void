/* @type {require('stylelint').Config} */
module.exports = {
  customSyntax: "postcss-less",
  extends: ["stylelint-config-recess-order", "stylelint-config-standard-less"],
  plugins: ["stylelint-order"],
  rules: {
    "no-descending-specificity": null,
    "selector-class-pattern": null,
    "media-query-no-invalid": null,
    "function-no-unknown": null,
    "less/no-duplicate-variables": null,
    "import-notation": null,
  },
};
