const ignoreAtRules = [
  'apply',
  'screen',
  'include',
  'plugin',
  'use',
  'mixin',
  'define-mixin',
];

module.exports = {
  extends: ['stylelint-config-standard'],
  ignorePath: 'old_blog/**',
  rules: {
    'import-notation': null,
    'at-rule-no-deprecated': [
      true,
      {
        ignoreAtRules,
      },
    ],
    'at-rule-no-unknown': [
      true,
      {
        ignoreAtRules,
      },
    ],
    'no-descending-specificity': null,
    'no-duplicate-selectors': null,
  },
};
