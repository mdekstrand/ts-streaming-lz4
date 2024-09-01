use wasm_bindgen::prelude::*;

use lz4_flex::block::compress;

pub mod error;
pub mod stream;

/// Compress a block of data in the LZ4 block compression format.
#[wasm_bindgen]
pub fn block_compress(bytes: Vec<u8>) -> Vec<u8> {
    return compress(&bytes);
}
