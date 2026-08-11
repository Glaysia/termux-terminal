import { readFile } from "node:fs/promises";

const paths = ["manifest.json", "packages/obsidian-plugin/manifest.json"];
const manifests = await Promise.all(
  paths.map(async (path) => ({ path, value: JSON.parse(await readFile(path, "utf8")) })),
);
const packageJson = JSON.parse(
  await readFile("packages/obsidian-plugin/package.json", "utf8"),
);
const [rootManifest, pluginManifest] = manifests.map(({ value }) => value);

for (const { path, value } of manifests) {
  if (!/^\d+\.\d+\.\d+$/.test(value.version)) {
    throw new Error(`${path} must use a x.y.z version`);
  }
  if (value.id.includes("obsidian")) {
    throw new Error(`${path} ID must not contain "obsidian"`);
  }
}

if (rootManifest.id !== pluginManifest.id) {
  throw new Error("Root and plugin manifest IDs must match");
}
if (rootManifest.version !== pluginManifest.version || pluginManifest.version !== packageJson.version) {
  throw new Error("Root manifest, plugin manifest, and package versions must match");
}

console.log(`Release metadata verified: ${rootManifest.id} v${rootManifest.version}`);
