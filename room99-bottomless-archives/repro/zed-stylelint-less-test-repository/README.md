# Test repository for LESS support
This repository must display lint errors in `src/main.less`, you can see them in console using `npm run lint-check:styles`.

```sh
> zed-stylelint-less-test-repository@1.0.0 lint-check:styles
> stylelint "src/**/*.(less|sass|scss|css)" --max-warnings=0 --allow-empty-input


src/main.less
  3:3  ✖  Expected "font-size" to come before "font-weight"  order/properties-order
  6:1  ✖  Unexpected unknown type selector "d"               selector-type-no-unknown

✖ 2 problems (2 errors, 0 warnings)
```
