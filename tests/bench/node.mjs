import { readFile, writeFile } from "node:fs/promises";
import { pathToFileURL } from "node:url";
import { join } from "node:path";
import { measure } from "./typescript.mjs";
const [run, op, size, n] = process.argv.slice(2),
  g = await import(pathToFileURL(process.env.TDF_TS_NODE_PACKAGE ?? process.env.TDF_TS_PACKAGE).href),
  raw = JSON.parse(await readFile(join(run, "private.json"))),
  input = new Uint8Array(await readFile(join(run, size + ".input"))),
  archive =
    op === "decrypt"
      ? new Uint8Array(await readFile(join(run, size + ".reference.tdf")))
      : null;
console.log(
  JSON.stringify(
    await measure(g, raw, input, archive, op, Number(n), (i, b) =>
      writeFile(join(run, `typescript-${size}-${i}.tdf`), b),
    ),
  ),
);
