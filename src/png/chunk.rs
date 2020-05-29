use std::fmt;
use std::io::{Read, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
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

    pub fn from_bytes(data_length: u32, bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() < 8 {
            anyhow::bail!("Invalid chunk")
        } else {
            let mut reader = BufReader::new(bytes);
            let mut buffer: [u8; 4] = [0, 0, 0, 0];

            reader.read_exact(&mut buffer)?;
            let chunk_type = ChunkType::from_bytes(buffer);

            let mut data: Vec<u8> =  vec![0; data_length as usize];
            reader.read_exact(&mut data)?;

            reader.read_exact(&mut buffer)?;
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
        write!(f, "{}", std::str::from_utf8(&self.bytes).expect("This should already be validated as ASCII"))
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let bytes = s.as_bytes();
        if bytes.len() == 4 && s.is_ascii() {
            Ok(Self::from_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        } else {
            anyhow::bail!("String must be 4 ASCII bytes")
        }
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
