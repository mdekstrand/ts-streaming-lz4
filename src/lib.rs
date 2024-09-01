use std::io::{Read, Write};

use error::StreamError;
use wasm_bindgen::prelude::*;

use lz4_flex::{block::compress, frame::FrameEncoder};

pub mod error;
pub mod stream;

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
import type { WriterSync } from "jsr:@std/io@^0.224/types";
"#;

/// Compress a block of data in the LZ4 block compression format.
#[wasm_bindgen]
pub fn compress_block(bytes: Vec<u8>) -> Vec<u8> {
    return compress(&bytes);
}

/// Compress data all at once, but using the streaming format.
#[wasm_bindgen]
pub fn compress_framed(bytes: Vec<u8>) -> Result<Vec<u8>, StreamError> {
    let out = Vec::<u8>::new();
    let mut enc = FrameEncoder::new(out);
    enc.write_all(&bytes)?;
    let out = enc.finish()?;
    Ok(out)
}
