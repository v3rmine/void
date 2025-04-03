module.exports = {
  customSyntax: "postcss-less",
  extends: [
    "stylelint-config-standard",
    "stylelint-config-recess-order",
    "stylelint-config-prettier",
  ],
  plugins: ["stylelint-order"],
  rules: {
    'at-rule-no-unknown': [
      true,
      { ignoreAtRules: [
        'tailwind',
        'apply',
        'variants',
        'responsive',
        'screen'
      ] }
    ],
    'declaration-block-trailing-semicolon': null,
    'no-descending-specificity': null
  }
};
