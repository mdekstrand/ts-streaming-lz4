use std::io::{self, Write};

use error::LZ4Error;
use wasm_bindgen::prelude::*;

use lz4_flex::{
    compress_prepend_size, decompress_size_prepended,
    frame::{FrameDecoder, FrameEncoder},
};

pub mod error;
pub mod stream;

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
import type { WriterSync } from "@std/io/types";
"#;

/// Compress a block of data in the LZ4 block compression format.
#[wasm_bindgen]
pub fn compress_block(bytes: Vec<u8>) -> Result<Vec<u8>, LZ4Error> {
    if bytes.len() > 4 * 1024 * 1024 {
        Err(LZ4Error::Oversized)
    } else {
        Ok(compress_prepend_size(&bytes))
    }
}

/// Decompress a block of data in the LZ4 block compression format.
#[wasm_bindgen]
pub fn decompress_block(bytes: Vec<u8>) -> Result<Vec<u8>, LZ4Error> {
    Ok(decompress_size_prepended(&bytes)?)
}

/// Compress data all at once, but using the streaming format.
#[wasm_bindgen]
pub fn compress_framed(bytes: Vec<u8>) -> Result<Vec<u8>, LZ4Error> {
    let out = Vec::<u8>::new();
    let mut enc = FrameEncoder::new(out);
    enc.write_all(&bytes)?;
    let out = enc.close()?;
    Ok(out)
}

/// Decompress data all at once, but using the streaming format.
#[wasm_bindgen]
pub fn decompress_framed(bytes: Vec<u8>) -> Result<Vec<u8>, LZ4Error> {
    let slice = bytes.as_slice();
    let mut out = Vec::<u8>::new();
    let mut dec = FrameDecoder::new(slice);
    io::copy(&mut dec, &mut out)?;
    Ok(out)
}
