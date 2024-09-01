use std::io;

use lz4_flex::frame::{FrameDecoder, FrameEncoder};
use streaming_lz4::error::StreamError;

fn main() -> Result<(), StreamError> {
    let args: Vec<_> = std::env::args().collect();
    let decompress = args.iter().any(|a| a == "-d");

    let mut input = std::io::stdin();
    let mut output = std::io::stdout();

    if decompress {
        let mut input = FrameDecoder::new(input);
        io::copy(&mut input, &mut output)?;
    } else {
        let mut output = FrameEncoder::new(output);
        io::copy(&mut input, &mut output)?;
    }

    Ok(())
}
