use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("LZ4 streaming error")]
    LZ4Error(#[from] lz4_flex::frame::Error),
    #[error("LZ4 block decompression error")]
    Decompress(#[from] lz4_flex::block::DecompressError),
}

impl Into<JsValue> for StreamError {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}
