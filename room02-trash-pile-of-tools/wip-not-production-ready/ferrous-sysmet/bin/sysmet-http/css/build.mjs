import { $ } from "zx";
import chokidar from "chokidar";
import { basename } from "path";
import { existsSync, mkdirSync } from "fs";
import { rm } from "fs/promises";
import "dotenv/config";

const { BUILD_ONLY, BUILD_SERVE, BUILD_WATCH, WITH_TIMINGS } = process.env;
// Because default quoting does not work on windows cf https://github.com/google/zx/issues/298
$.quote = (s) => s;

const IS_SERVING = BUILD_SERVE === "true";
const IS_WATCHING = BUILD_WATCH === "true";
const IS_RELEASE = !IS_SERVING && !IS_WATCHING;
const TIMED = WITH_TIMINGS === "true" ? "time " : "";
const OUTPUT_DIR = "exports";

/**
 * Compiles CSS
 * */
async function css() {
  const compileFile = async (path, withPostcss) => {
    await $`if [ ! -d .stage-css.tmp ]; then mkdir .stage-css.tmp; fi`
    await $`${TIMED}less-watch-compiler --run-once styles .stage-css.tmp ${path ? basename(path) : ""}`;
    if (withPostcss) {
      await $`${TIMED}${IS_RELEASE ? "NODE_ENV=production " : ""}postcss '.stage-css.tmp/*.css' --dir ${OUTPUT_DIR} --config postcss.config.js`;
    } else {
      await $`${TIMED} cp -r .stage-css.tmp/*.css ${OUTPUT_DIR}/`;
    }
    await $`rm -rf .stage-css.tmp`;
  };

  // if (IS_SERVING || IS_WATCHING) await compileFile("tailwind.less", true);
  // await $`cp node_modules/\@picocss/pico/css/pico.min.css exports/pico.min.css`;

  IS_SERVING || IS_WATCHING
    ? chokidar
    .watch("styles/**/*.less", { followSymlinks: false })
    .on("change", () => compileFile(null))
    .on("unlink", (path) => {
      if (path.match(/^styles\/[^\/]+\.less$/)) rm(`${OUTPUT_DIR}/${basename(path)}`);
    })
    : await compileFile(null, true)
}

/**
 * Compiles Typescript
 * */
async function typescript() {
  const compiteTs = async () => {
    try {
      // NOTE: Disabled for now because it is not usefull
      // await $`${TIMED}esbuild src/js-services/main.ts --format=iife --bundle --outfile="dist/js-services.js" ${IS_RELEASE ? ["--minify", " --target=es2015"] : []} --platform=browser`;  
    } catch {
      // NOTE: Catch errors
    }
  };

  IS_SERVING || IS_WATCHING
    ? chokidar.watch("src/**/*.{ts,js}", { followSymlinks: false })
    .on("add", () => compiteTs())
    .on("change", () => compiteTs())
    .on("unlink", async (path) => {
      rm(`${OUTPUT_DIR}/${basename(path)}`);
    })
    : await compiteTs();
}

/**
 * Initializes build
 * */
async function buildAll() {
  try {
    await Promise.all([css(), typescript()]);
  } catch (err) {
    console.log(err);
  }
}

if (!existsSync(OUTPUT_DIR)) {
  mkdirSync(OUTPUT_DIR);
}

switch (BUILD_ONLY) {
  case "css":
    css();
    break;
  case "javascript":
    typescript();
    break;
  default:
    buildAll();
    break;
}
