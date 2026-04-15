const AutoPrefixerPlugin = require("autoprefixer");
const PurgeCssPlugin = require("@fullhuman/postcss-purgecss");
const CssNanoPlugin = require("cssnano");

const IS_PROD = process.env.NODE_ENV === "production";

module.exports = {
  debug: !IS_PROD,
  plugins: IS_PROD
      ? [
          CssNanoPlugin({
            preset: [
              "default",
              {
                discardComments: {
                  removeAll: true,
                },
                autoprefixer: false,
                discardUnused: true,
                mergeIdents: true,
                reduceIdents: true,
                zindex: true,
              },
            ],
          }),
          PurgeCssPlugin({
            content: ["styles/**/*"],
            keyframes: true,
            fontFace: true,
          }),
          AutoPrefixerPlugin,
        ]
      : [],
};
