import esbuild from "esbuild";
import { appendFile, readFile, writeFile } from "node:fs/promises";

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

const xtermStyles = await readFile("node_modules/@xterm/xterm/css/xterm.css", "utf8");
const compatibleXtermStyles = xtermStyles
  .replace("opacity: 1 !important;", "opacity: 1;")
  .replace("font-size: 11px !important;", "font-size: 11px;")
  .replaceAll("text-decoration: double underline;", "text-decoration: underline;")
  .replaceAll("text-decoration: wavy underline;", "text-decoration: underline;")
  .replaceAll("text-decoration: dotted underline;", "text-decoration: underline;")
  .replaceAll("text-decoration: dashed underline;", "text-decoration: underline;")
  .replaceAll("text-decoration: overline double underline;", "text-decoration: overline;")
  .replaceAll("text-decoration: overline wavy underline;", "text-decoration: overline;")
  .replaceAll("text-decoration: overline dotted underline;", "text-decoration: overline;")
  .replaceAll("text-decoration: overline dashed underline;", "text-decoration: overline;")
  .replaceAll("text-decoration: overline underline;", "text-decoration: overline;");
await writeFile("styles.css", compatibleXtermStyles);
await appendFile("styles.css", "\n");
await appendFile("styles.css", await readFile("src/styles.css"));
