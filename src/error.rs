use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum LZ4Error {
    #[error("input block too large")]
    Oversized,
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("LZ4 streaming error")]
    FrameError(#[from] lz4_flex::frame::Error),
    #[error("LZ4 block decompression error")]
    Decompress(#[from] lz4_flex::block::DecompressError),
}

impl Into<JsValue> for LZ4Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}
