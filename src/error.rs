use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("LZ4 encoding error")]
    LZ4Error(#[from] lz4_flex::frame::Error),
}

impl Into<JsValue> for StreamError {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}
