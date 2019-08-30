use std::fmt;

use crate::{Error, Result};
use crate::utils::{four_bytes, u32_from_slice_range};

#[derive(Debug)]
pub struct Png {
    chunk: Vec<Chunk>,
}

impl Png {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 12 {
            Err(Error::new("Invalid chunk"))
        } else {
            let length = u32_from_slice_range(bytes, 0..4)?;
            let chunk_type = ChunkType::from_bytes(four_bytes(bytes, 5..8)?);
            let data = bytes[8..length as usize].to_vec();

            let crc_start = 8 + length as usize;
            let crc_end = crc_start + 4;
            let crc = u32_from_slice_range(bytes, crc_start..crc_end)?;

            Ok(Self { length, chunk_type, data, crc })
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{", )?;
        writeln!(f, "  Length: {}", self.length)?;
        writeln!(f, "  Type: {}", self.chunk_type)?;
        writeln!(f, "  Data: {} bytes", self.data.len())?;
        writeln!(f, "  Crc: {}", self.crc)?;
        writeln!(f, "}}", )?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        Self { bytes }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        if bytes.len() == 4 && s.is_ascii() {
            Ok(Self::from_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        } else {
            Err(Error::new("String must be 4 ASCII bytes"))
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    pub fn is_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes).expect("This is already validated as ASCII"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_type() -> ChunkType {
        ChunkType::from_bytes([82, 117, 83, 116])
    }

    #[test]
    pub fn test_chunk_type_from_bytes() {
        assert_eq!(test_type().bytes(), &[82, 117, 83, 116]);
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(chunk, test_type());
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        assert!(test_type().is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        assert!(!test_type().is_public());
    }

    #[test]
    pub fn test_chunk_type_is_valid() {
        assert!(test_type().is_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        assert!(test_type().is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_string() {
        assert_eq!(&test_type().to_string(), "RuSt");
    }
}
