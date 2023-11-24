import * as fs from "node:fs/promises";
import * as path from "node:path";
import { execSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const examplesPath = path.resolve(__dirname, "../examples");
const items = await fs.readdir(examplesPath);

const exists = (p) =>
  fs.access(p).then(
    () => true,
    () => false
  );

for (const item of items) {
  const examplePath = path.resolve(examplesPath, item);
  const exec = (command) =>
    execSync(command, { cwd: examplePath, stdio: "inherit" });

  if (await exists(path.resolve(examplePath, "Cargo.toml"))) {
    exec("wasm-pack build --target web");
  }

  if (await exists(path.resolve(examplePath, "package.json"))) {
    exec("npm install");
    exec("npm run build");
  }
}
