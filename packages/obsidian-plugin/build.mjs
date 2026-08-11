import esbuild from "esbuild";
import { appendFile, copyFile, readFile } from "node:fs/promises";

const packageJson = JSON.parse(await readFile("package.json", "utf8"));
const manifest = JSON.parse(await readFile("manifest.json", "utf8"));
const rootManifest = JSON.parse(await readFile("../../manifest.json", "utf8"));
if (
  packageJson.version !== manifest.version ||
  manifest.version !== rootManifest.version ||
  manifest.id !== rootManifest.id
) {
  throw new Error(
    "Plugin package, plugin manifest, and root manifest must use the same ID and version",
  );
}

await esbuild.build({
  entryPoints: ["src/main.ts"],
  bundle: true,
  external: ["obsidian", "electron", "@codemirror/autocomplete", "@codemirror/collab", "@codemirror/commands", "@codemirror/language", "@codemirror/lint", "@codemirror/search", "@codemirror/state", "@codemirror/view", "@lezer/common", "@lezer/highlight", "@lezer/lr"],
  format: "cjs",
  outfile: "main.js",
  platform: "browser",
  target: "es2022",
  sourcemap: false,
  define: {
    __PLUGIN_VERSION__: JSON.stringify(packageJson.version),
  },
});

await copyFile("node_modules/@xterm/xterm/css/xterm.css", "styles.css");
await appendFile("styles.css", "\n");
await appendFile("styles.css", await readFile("src/styles.css"));
