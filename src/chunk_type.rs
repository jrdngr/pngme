use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.clone()
    }

    /// Returns the property state of the first byte as described in the PNG spec
    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    /// Returns the property state of the second byte as described in the PNG spec
    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    /// Returns the property state of the third byte as described in the PNG spec
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented
    /// by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    #[rustfmt::skip]
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() &&
        ChunkType::is_valid_byte(self.bytes[0]) &&
        ChunkType::is_valid_byte(self.bytes[1]) &&
        ChunkType::is_valid_byte(self.bytes[2]) &&
        ChunkType::is_valid_byte(self.bytes[3])
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    #[rustfmt::skip]
    pub fn is_valid_byte(byte: u8) -> bool {
        (byte >= 65 && byte <= 90) ||
        (byte >= 97 && byte <= 122)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(bytes: [u8; 4]) -> anyhow::Result<Self> {
        for byte in bytes.iter() {
            if !ChunkType::is_valid_byte(*byte) {
                anyhow::bail!(
                    "Invalid byte {}. Valid bytes are ASCII A-Z and a-z, or 65-90 and 97-122",
                    *byte
                );
            }
        }

        Ok(Self { bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.bytes).expect("This is already validated as ASCII")
        )
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let bytes = s.as_bytes();

        if bytes.len() == 4 && s.is_ascii() {
            Ok(Self::try_from([bytes[0], bytes[1], bytes[2], bytes[3]])?)
        } else {
            anyhow::bail!("String must be 4 ASCII bytes")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());

        let bytes = "Rust".as_bytes();
        let bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let chunk = ChunkType { bytes };
        assert!(!chunk.is_valid());

        let bytes = "Ru1t".as_bytes();
        let bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let chunk = ChunkType { bytes };
        assert!(!chunk.is_valid());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }
}
