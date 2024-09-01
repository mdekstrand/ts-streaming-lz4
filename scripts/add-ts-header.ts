#!/usr/bin/env -S deno run --allow-read=pkg/ --allow-write=pkg/
import { parse as parsePath } from "jsr:@std/path@1.0";
import { writeAll } from "jsr:@std/io@^0.224";

const encoder = new TextEncoder();

for (const mod of Deno.args) {
  let parsed = parsePath(mod);
  console.info("fixing JS script %s", parsed.name);
  const text = await Deno.readFile(mod);
  using file = await Deno.open(mod, { write: true, truncate: true });
  await writeAll(
    file,
    encoder.encode(`// @ts-self-types="./${parsed.name}.d.ts"\n`),
  );
  await writeAll(file, text);
}
