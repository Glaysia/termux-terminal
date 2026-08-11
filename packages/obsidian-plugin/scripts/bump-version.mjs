import { readFile, writeFile } from "node:fs/promises";

const kind = process.argv[2] ?? "patch";
if (!["major", "minor", "patch"].includes(kind)) {
  throw new Error("Usage: node scripts/bump-version.mjs [major|minor|patch]");
}

const packageJson = JSON.parse(await readFile("package.json", "utf8"));
const manifest = JSON.parse(await readFile("manifest.json", "utf8"));
const rootManifest = JSON.parse(await readFile("../../manifest.json", "utf8"));
const rootPackageJson = JSON.parse(await readFile("../../package.json", "utf8"));
const versions = JSON.parse(await readFile("../../versions.json", "utf8"));
const cargoToml = await readFile("../../Cargo.toml", "utf8");
const cargoVersionPattern = /(^\[workspace\.package\][\s\S]*?^version = )"([^"]+)"/m;
const cargoVersion = cargoToml.match(cargoVersionPattern)?.[2];
if (
  packageJson.version !== manifest.version ||
  manifest.version !== rootManifest.version ||
  manifest.id !== rootManifest.id ||
  rootPackageJson.version !== manifest.version ||
  cargoVersion !== manifest.version ||
  versions[manifest.version] !== manifest.minAppVersion
) {
  throw new Error("release metadata must match before bumping");
}

const [major, minor, patch] = packageJson.version.split(".").map(Number);
if (![major, minor, patch].every(Number.isInteger)) {
  throw new Error(`Unsupported version: ${packageJson.version}`);
}

const next =
  kind === "major"
    ? `${major + 1}.0.0`
    : kind === "minor"
      ? `${major}.${minor + 1}.0`
      : `${major}.${minor}.${patch + 1}`;

packageJson.version = next;
manifest.version = next;
rootManifest.version = next;
rootPackageJson.version = next;
versions[next] = manifest.minAppVersion;
await writeFile("package.json", `${JSON.stringify(packageJson, null, 2)}\n`);
await writeFile("manifest.json", `${JSON.stringify(manifest, null, 2)}\n`);
await writeFile("../../manifest.json", `${JSON.stringify(rootManifest, null, 2)}\n`);
await writeFile("../../package.json", `${JSON.stringify(rootPackageJson, null, 2)}\n`);
await writeFile("../../versions.json", `${JSON.stringify(versions, null, 2)}\n`);
await writeFile(
  "../../Cargo.toml",
  cargoToml.replace(cargoVersionPattern, `$1"${next}"`),
);
console.log(next);
