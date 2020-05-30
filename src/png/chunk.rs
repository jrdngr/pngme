use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use super::ChunkType;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc_data: Vec<u8> = chunk_type
            .bytes()
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

    pub fn from_bytes(data_length: u32, bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() < 8 {
            anyhow::bail!("Invalid chunk")
        } else {
            let mut reader = BufReader::new(bytes);
            let mut buffer: [u8; 4] = [0, 0, 0, 0];

            reader.read_exact(&mut buffer)?;
            let chunk_type = ChunkType::try_from(buffer)?;

            let mut data: Vec<u8> = vec![0; data_length as usize];
            reader.read_exact(&mut data)?;

            reader.read_exact(&mut buffer)?;
            let crc = u32::from_be_bytes(buffer);

            Ok(Self {
                length: data_length,
                chunk_type,
                data,
                crc,
            })
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
        self.length
            .to_be_bytes()
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
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length)?;
        writeln!(f, "  Type: {}", self.chunk_type)?;
        writeln!(f, "  Data: {} bytes", self.data.len())?;
        writeln!(f, "  Crc: {}", self.crc)?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

