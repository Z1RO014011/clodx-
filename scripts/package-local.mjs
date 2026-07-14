import { cp, mkdir, readdir, rm } from "node:fs/promises";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

const platform = process.argv[2];
const root = process.cwd();
const bundleDir = join(root, "src-tauri", "target", "release", "bundle");
const releaseDir = join(root, "release");

function run(command, args) {
  const result = spawnSync(command, args, { stdio: "inherit" });
  if (result.status !== 0) {
    throw new Error(`${command} failed`);
  }
}

async function resetReleaseDirectory() {
  await rm(releaseDir, { recursive: true, force: true });
  await mkdir(releaseDir, { recursive: true });
}

async function copyWindowsInstallers(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const source = join(directory, entry.name);
    if (entry.isDirectory()) {
      await copyWindowsInstallers(source);
    } else if (entry.name.endsWith(".msi") || entry.name.endsWith(".exe")) {
      await cp(source, join(releaseDir, entry.name));
    }
  }
}

async function packageMac() {
  if (process.platform !== "darwin") {
    throw new Error("package:mac must run on macOS");
  }

  const app = join(bundleDir, "macos", "Clodx.app");
  const architecture = process.arch === "arm64" ? "arm64" : "x64";
  const zip = join(releaseDir, `Clodx-macos-${architecture}.app.zip`);
  const dmg = join(releaseDir, `Clodx-macos-${architecture}.dmg`);

  run("codesign", ["--force", "--deep", "--sign", "-", app]);
  run("codesign", ["--verify", "--deep", "--strict", "--verbose=2", app]);
  run("ditto", ["-c", "-k", "--sequesterRsrc", "--keepParent", app, zip]);
  run("hdiutil", ["create", "-volname", "Clodx", "-srcfolder", app, "-ov", "-format", "UDZO", dmg]);
}

async function packageWindows() {
  if (process.platform !== "win32") {
    throw new Error("package:win must run on Windows");
  }

  await copyWindowsInstallers(bundleDir);
}

await resetReleaseDirectory();

if (platform === "mac") {
  await packageMac();
} else if (platform === "win") {
  await packageWindows();
} else {
  throw new Error("Use `npm run package:mac` or `npm run package:win`");
}

console.log(`Installers are ready in ${releaseDir}`);
