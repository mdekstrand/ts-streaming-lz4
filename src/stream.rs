use std::io;
use std::io::Write;

use wasm_bindgen::prelude::*;

use lz4_flex::frame::FrameEncoder;

use crate::error::LZ4Error;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "WriterSync")]
    pub type WriterSync;

    #[wasm_bindgen(method, structural, catch)]
    fn writeSync(this: &WriterSync, data: &[u8]) -> Result<usize, JsValue>;
}

/// Stream-format LZ4 encoder.
///
/// A stream encoder that implements Deno's `WriterSync` writes to an underlying
/// `WriterSync`.
#[wasm_bindgen]
pub struct LZ4EncoderStream {
    encoder: FrameEncoder<JSWriter>,
}

#[wasm_bindgen]
impl LZ4EncoderStream {
    /// Construct a new LZ4 encoder stream from a Deno `WriterSync`.
    #[wasm_bindgen(constructor)]
    pub fn new(out: WriterSync) -> LZ4EncoderStream {
        let out = JSWriter::new(out);
        let encoder = FrameEncoder::new(out);
        LZ4EncoderStream { encoder }
    }

    #[allow(non_snake_case)]
    pub fn writeSync(&mut self, data: &[u8]) -> Result<usize, LZ4Error> {
        Ok(self.encoder.write(data)?)
    }

    /// Finish the underlying LZ4 stream and release the encoder.
    pub fn finish(self) -> Result<(), LZ4Error> {
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
