/**
 * Benchmark compressing the compressor's WASM file against several other compressors.
 * @module
 */

import { Buffer, writeAllSync } from "jsr:@std/io@^0.224";
import { Buffer as StreamBuffer } from "jsr:@std/streams@^1.0";

import { compress_block, compress_framed, LZ4EncoderStream } from "../mod.ts";
import * as denosaurs from "jsr:@denosaurs/lz4@^0.1.4";
import * as node_wasm from "npm:lz4-wasm-nodejs@^0.9.2";

const data = await Deno.readFile(
  "pkg/streaming_lz4_bg.wasm",
);

Deno.bench("compress_block", { baseline: true }, () => {
  compress_block(data);
});

Deno.bench("compress_framed", () => {
  compress_framed(data);
});

Deno.bench("encoder stream", () => {
  const output = new Buffer();
  const enc = new LZ4EncoderStream(output);
  writeAllSync(enc, data);
  enc.finish();
});

Deno.bench("lz4-wasm-nodejs", () => {
  node_wasm.compress(data);
});
Deno.bench("@denosaurs", () => {
  denosaurs.compress(data);
});

Deno.bench("gzip", async () => {
  const input = new StreamBuffer(data);
  const output = new StreamBuffer();
  await input.readable.pipeThrough(new CompressionStream("gzip")).pipeTo(
    output.writable,
  );
});
