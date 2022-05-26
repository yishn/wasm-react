import * as fs from "node:fs/promises";
import * as path from "node:path";
import { execSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const examplesPath = path.resolve(__dirname, "../examples");
const items = await fs.readdir(examplesPath);

for (const item of items) {
  const examplePath = path.resolve(examplesPath, item);
  const stat = await fs.stat(path.resolve(examplePath, "Cargo.toml"));

  if (stat.isFile()) {
    execSync("wasm-pack build --target web", { cwd: examplePath });
  }
}
