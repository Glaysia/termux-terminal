import { readFile } from "node:fs/promises";

const paths = ["manifest.json", "packages/obsidian-plugin/manifest.json"];
const manifests = await Promise.all(
  paths.map(async (path) => ({ path, value: JSON.parse(await readFile(path, "utf8")) })),
);
const packageJson = JSON.parse(
  await readFile("packages/obsidian-plugin/package.json", "utf8"),
);
const rootPackageJson = JSON.parse(await readFile("package.json", "utf8"));
const versions = JSON.parse(await readFile("versions.json", "utf8"));
const cargoToml = await readFile("Cargo.toml", "utf8");
const [rootManifest, pluginManifest] = manifests.map(({ value }) => value);
const cargoVersion = cargoToml.match(/(^\[workspace\.package\][\s\S]*?^version = )"([^"]+)"/m)?.[2];

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
if (
  rootManifest.version !== pluginManifest.version ||
  pluginManifest.version !== packageJson.version ||
  packageJson.version !== rootPackageJson.version ||
  rootPackageJson.version !== cargoVersion
) {
  throw new Error("Plugin, root package, manifest, and bridge versions must match");
}
if (versions[rootManifest.version] !== rootManifest.minAppVersion) {
  throw new Error("versions.json must map the current plugin version to minAppVersion");
}

console.log(`Release metadata verified: ${rootManifest.id} v${rootManifest.version}`);
