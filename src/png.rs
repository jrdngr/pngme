use std::fmt;
use std::io::{Read, BufReader};

use crate::{Error, Result};

#[derive(Debug)]
pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(bytes);
        let mut header: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut chunks = Vec::new();

        reader.read_exact(&mut header).unwrap();

        let mut length_buffer: [u8; 4] = [0, 0, 0, 0];
        while let Ok(()) = reader.read_exact(&mut length_buffer) {
            let length = u32::from_be_bytes(length_buffer);

            let chunk_length = (length + 8) as usize;
            let mut chunk_data: Vec<u8> = vec![0; chunk_length];
            reader.read_exact(&mut chunk_data).unwrap();
            
            let chunk = Chunk::from_bytes(length, &chunk_data).unwrap();
            chunks.push(chunk);
        }

        Ok(Self {
            header,
            chunks,
        })
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        if !self.chunks.is_empty() {
            let index = self.chunks.len() - 1;
            self.chunks.insert(index, chunk);
        }
    }

    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        unimplemented!()
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        match ChunkType::from_str(chunk_type) {
            Ok(chunk_type) => {
                for chunk in &self.chunks {
                    if chunk.chunk_type == chunk_type {
                        return Some(&chunk);
                    }
                }
                None
            },
            Err(e) => None,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend(&self.header);
        for chunk in &self.chunks {
            result.extend(chunk.as_bytes());
        }

        result
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Header: {:?}", self.header)?;
        for chunk in &self.chunks {
            writeln!(f, "{}", chunk)?;
        }
        Ok(())
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
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc_data: Vec<u8> = chunk_type.bytes()
            .iter()
            .cloned()
            .chain(data.iter().cloned())
            .collect();
        let crc = crc::crc32::checksum_ieee(&crc_data);

        Self {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn from_bytes(data_length: u32, bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            Err(Error::new("Invalid chunk"))
        } else {
            let mut reader = BufReader::new(bytes);
            let mut buffer: [u8; 4] = [0, 0, 0, 0];

            reader.read_exact(&mut buffer).expect("Failed to read chunk type");
            let chunk_type = ChunkType::from_bytes(buffer);

            let mut data: Vec<u8> =  vec![0; data_length as usize];
            reader.read_exact(&mut data).expect("Failed to read chunk data");

            reader.read_exact(&mut buffer).expect("Failed to read crc");
            let crc = u32::from_be_bytes(buffer);

            Ok(Self { length: data_length, chunk_type, data, crc })
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

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length.to_be_bytes()
            .iter()
            .cloned()
            .chain(self.chunk_type().bytes().iter().cloned())
            .chain(self.data.iter().cloned())
            .chain(self.crc.to_be_bytes().iter().cloned())
            .collect()
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
