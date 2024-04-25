import fs from "node:fs";
import { join } from "node:path";
await Bun.build({
    entrypoints: ["."],
    outdir: "./build",
});

fs.copyFileSync(join(process.cwd(), "plugin.json"), join(process.cwd(), "build", "plugin.json"));
fs.copyFileSync(join(process.cwd(), "readme.md"), join(process.cwd(), "build", "readme.md"));
