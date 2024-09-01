/**
 * Benchmark compressing the compressor's WASM file against several other compressors.
 * @module
 */

import { Buffer, writeAllSync } from "jsr:@std/io@^0.224";
import { Buffer as StreamBuffer } from "jsr:@std/streams@^1.0";

import { compress_block, compress_framed, LZ4EncoderStream } from "../mod.ts";
import * as denosaurs from "jsr:@denosaurs/lz4@^0.1.4";
import * as node_wasm from "npm:lz4-wasm-nodejs@^0.9.2";

const files = {
  self: await Deno.readFile(
    "pkg/streaming_lz4_bg.wasm",
  ),
  cli: await Deno.readFile("target/debug/streaming-lz4"),
  deno: await Deno.readFile(Deno.execPath()),
};

for (const [name, data] of Object.entries(files)) {
  console.info("source %s: %d bytes", name, data.length);

  if (name != "deno") {
    Deno.bench("compress_block", { group: name, baseline: true }, () => {
      compress_block(data);
    });
  }

  Deno.bench("compress_framed", { group: name }, () => {
    compress_framed(data);
  });

  Deno.bench("encoder stream", { group: name }, () => {
    const output = new Buffer();
    const enc = new LZ4EncoderStream(output);
    writeAllSync(enc, data);
    enc.finish();
  });

  Deno.bench("lz4-wasm-nodejs", { group: name }, () => {
    node_wasm.compress(data);
  });
  Deno.bench("@denosaurs", { group: name }, () => {
    denosaurs.compress(data);
  });

  Deno.bench("gzip", { group: name }, async () => {
    const input = new StreamBuffer(data);
    const output = new StreamBuffer();
    await input.readable.pipeThrough(new CompressionStream("gzip")).pipeTo(
      output.writable,
    );
  });
}
