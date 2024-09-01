use wasm_bindgen::prelude::*;

use lz4_flex::block::compress;

pub mod error;
pub mod stream;

#[wasm_bindgen]
pub fn block_compress(bytes: Vec<u8>) -> Vec<u8> {
    return compress(&bytes);
}
