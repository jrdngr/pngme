pub mod chunk;
pub mod chunk_type;

use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

pub use chunk::Chunk;
pub use chunk_type::ChunkType;

#[derive(Debug)]
pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    pub const EXPECTED_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let bytes = fs::read(path)?;
        Ok(Self::try_from(bytes.as_ref())?)
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        if !self.chunks.is_empty() {
            let index = self.chunks.len() - 1;
            self.chunks.insert(index, chunk);
        }
    }

    pub fn remove_chunk(&mut self, chunk_type: &str) -> anyhow::Result<Chunk> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let mut target_index: Option<usize> = None;
        for (index, chunk) in self.chunks.iter().enumerate() {
            if chunk.chunk_type == chunk_type {
                target_index = Some(index);
                break;
            }
        }

        match target_index {
            Some(index) => Ok(self.chunks.remove(index)),
            None => anyhow::bail!("Chunk not found"),
        }
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
            }
            Err(_) => None,
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

impl TryFrom<&[u8]> for Png {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> anyhow::Result<Png> {
        let mut reader = BufReader::new(bytes);
        let mut header: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut chunks = Vec::new();

        reader.read_exact(&mut header)?;

        if header != Png::EXPECTED_HEADER {
            anyhow::bail!("Invalid header: {:?}", header);
        }

        let mut length_buffer: [u8; 4] = [0, 0, 0, 0];
        while let Ok(()) = reader.read_exact(&mut length_buffer) {
            let length = u32::from_be_bytes(length_buffer);

            // Data length + 4 byte chunk type + 4 byte crc
            let chunk_length = (length + 8) as usize;
            
            let mut chunk_data: Vec<u8> = vec![0; chunk_length];
            reader.read_exact(&mut chunk_data)?;

            let chunk_bytes: Vec<u8> = length_buffer
                .iter()
                .copied()
                .chain(chunk_data.into_iter())
                .collect();

            let chunk = Chunk::try_from(chunk_bytes.as_ref())?;
            chunks.push(chunk);
        }

        Ok(Self { header, chunks })
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

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_chunks() -> Vec<Chunk> {
        let mut chunks = Vec::new();

        chunks.push(Chunk::from_strings("FrSt", "I am the first chunk").unwrap());
        chunks.push(Chunk::from_strings("miDl", "I am another chunk").unwrap());
        chunks.push(Chunk::from_strings("LASt", "I am the last chunk").unwrap());

        chunks
    }

    #[test]
    fn test_valid_from_bytes() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::EXPECTED_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());
        
        assert!(png.is_ok());
    }

    #[test]
    fn test_invalid_header() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> =[13, 80, 78, 71, 13, 10, 26, 10]
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());
        
        assert!(png.is_err());
    }
}
