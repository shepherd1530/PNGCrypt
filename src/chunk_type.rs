use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use anyhow::{Error, Result};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ChunkType {
    pub bytes: [u8; 4],
}


#[derive(Debug)]
pub enum ChunkTypeError {
    InconsistentByteLength(usize),
    InvalidChunkType(String),
}

impl std::error::Error for ChunkTypeError {}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::InconsistentByteLength(len) => {
                write!(f, "Inconsistent byte length: {}", len)
            }
            ChunkTypeError::InvalidChunkType(char) => write!(f, "Invalid chunk type: {}", char),
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self { bytes: value })
    }
}

 impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let values = value.as_bytes();

        let is_valid = &values
            .iter()
            .all(|&elem| (elem >= 65 && elem <= 90) || (elem >= 97 && elem <= 122));

        if !is_valid {
            return Err(ChunkTypeError::InvalidChunkType(value.to_string()).into());
        }

        if values.len() != 4 {
            return Err(ChunkTypeError::InconsistentByteLength(
                values.len(),
            ).into());
        }

        Ok(ChunkType::try_from([
            values[0], values[1], values[2], values[3],
        ])?)
    }
}


impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _str = std::str::from_utf8(&self.bytes).unwrap();
        write!(f, "{}", _str)
    }
}

impl Eq for ChunkType {}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl ChunkType {
   pub  const CHUNK_TYPE_SIZE:usize = 4;
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        self.bytes
            .iter()
            .all(|&elem| (elem >= 65 && elem <= 90) || (elem >= 97 && elem <= 122))
            && self.is_reserved_bit_valid() // check if the chunk type conforms with the png standard.
    }

    fn is_critical(&self) -> bool {
        self.bytes[0] >= 65 && self.bytes[0] <= 90
    }

    fn is_public(&self) -> bool {
        self.bytes[1] >= 65 && self.bytes[1] <= 90
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] >= 65 && self.bytes[2] <= 90
    }

    fn is_safe_to_copy(&self) -> bool {
        self.bytes[3] >= 97 && self.bytes[3] <= 122
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

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
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

