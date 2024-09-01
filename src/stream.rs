use std::io;
use std::io::Write;

use wasm_bindgen::prelude::*;

use lz4_flex::frame::FrameEncoder;

use crate::error::StreamError;

#[wasm_bindgen]
extern "C" {
    pub type WriterSync;

    #[wasm_bindgen(method, structural, catch)]
    fn writeSync(this: &WriterSync, data: &[u8]) -> Result<usize, JsValue>;
}

#[wasm_bindgen]
pub struct LZ4EncoderStream {
    encoder: FrameEncoder<JSWriter>,
}

#[wasm_bindgen]
impl LZ4EncoderStream {
    #[wasm_bindgen(constructor)]
    pub fn new(out: WriterSync) -> LZ4EncoderStream {
        let out = JSWriter::new(out);
        let encoder = FrameEncoder::new(out);
        LZ4EncoderStream { encoder }
    }

    #[allow(non_snake_case)]
    pub fn writeSync(&mut self, data: &[u8]) -> Result<usize, StreamError> {
        Ok(self.encoder.write(data)?)
    }

    pub fn finish(self) -> Result<(), StreamError> {
        self.encoder.finish()?;
        Ok(())
    }
}

struct JSWriter {
    jsout: WriterSync,
}

impl JSWriter {
    fn new(jsout: WriterSync) -> JSWriter {
        JSWriter { jsout }
    }
}

impl Write for JSWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.jsout.writeSync(buf) {
            Ok(x) => Ok(x),
            Err(_jsv) => Err(io::Error::other("asdf")),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        // flush does nothing
        Ok(())
    }
}
