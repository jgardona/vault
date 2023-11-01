use std::io::{prelude::*, Result};

use flate2::{
    read::{GzDecoder, GzEncoder},
    Compression,
};

pub fn compress(input: &[u8]) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut encoder = GzEncoder::new(input, Compression::default());
    encoder.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(input);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    Ok(buffer)
}

#[cfg(test)]
mod compression_tests {
    use anyhow::{Ok, Result};

    use crate::store::compression::decompress;

    use super::compress;

    const MESSAGE: &[u8; 43] = b"the quick brown fox jumps over the lazy dog";

    #[test]
    fn test_compress_decompress() -> Result<()> {
        let compressed = compress(MESSAGE)?;
        assert!(compressed.len() > 0);
        let decompressed = decompress(&compressed)?;

        assert_eq!(MESSAGE, &decompressed[..]);
        Ok(())
    }
}
