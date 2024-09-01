use wasm_bindgen::prelude::*;

use lz4_flex::block::compress;

pub mod error;
pub mod stream;

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
import type { WriterSync } from "jsr:@std/io@^0.224/types";
"#;

/// Compress a block of data in the LZ4 block compression format.
#[wasm_bindgen]
pub fn block_compress(bytes: Vec<u8>) -> Vec<u8> {
    return compress(&bytes);
}
