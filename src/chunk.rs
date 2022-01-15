use std::convert::{TryFrom, TryInto};
use std::fmt;

use super::chunk_type::ChunkType;

use anyhow::{Context, Error, Result};
use crc::crc32;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Chunk {
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub length: u32,
    pub crc: u32,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data_as_string().expect("Can not construct a valid string from Chunk."))
    }
}

#[derive(Debug)]
pub enum ChunkError {
    InvalidCrc(u32),
    InvalidLength(u32),
    InvalidChunkType(String),
}

impl std::error::Error for ChunkError {}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::InvalidCrc(crc) => write!(f, "Invalid crc: {}", crc),
            ChunkError::InvalidLength(len) => write!(f, "Invalid length: {}", len),
            ChunkError::InvalidChunkType(char) => write!(f, "Invalid chunk type: {}", char),
        }
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let length_data: [u8; 4] = value[0..Chunk::LENGTH_SIZE].try_into()?;
        let length:usize = u32::from_be_bytes(length_data).try_into()?;

        let chunk_type_data: [u8; 4] = value
            [Chunk::LENGTH_SIZE..Chunk::LENGTH_SIZE + ChunkType::CHUNK_TYPE_SIZE]
            .try_into()?;

        let chunk_type = ChunkType::try_from(chunk_type_data).context("Unable to construct chunk type from given data.")?;

        let data_last_index: usize = length + Chunk::LENGTH_SIZE + ChunkType::CHUNK_TYPE_SIZE;

        let crc = crc32::checksum_ieee(&value[Chunk::LENGTH_SIZE..data_last_index]); // we skip the chunk length
        let data: Vec<u8> = value[8..data_last_index].to_vec();

        if crc != u32::from_be_bytes(value[data_last_index..data_last_index + Chunk::CRC_SIZE].try_into()?) {
            return Err(ChunkError::InvalidCrc(crc).into());
        }

        if data.len() != length {
            return Err(ChunkError::InvalidLength(length as u32).into());
        }

        if !chunk_type.is_valid() {
            return Err(ChunkError::InvalidChunkType(chunk_type.to_string()).into());
        }

        Ok(Self {
            chunk_type: chunk_type,
            data: data,
            length: length as u32,
            crc: crc,
        })
    }
}

impl Chunk {
    const LENGTH_SIZE: usize = 4;
    const CRC_SIZE: usize = 4;

    fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(std::str::from_utf8(&self.data)?.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.chunk_type.bytes);
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&self.crc.to_be_bytes());

        bytes
    }

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let length = data.len() as u32;
        // join chunk type and data to calculate crc
        let chained = chunk_type
            .bytes
            .iter()
            .chain(&data)
            .cloned()
            .collect::<Vec<u8>>();
        let crc = crc32::checksum_ieee(&chained);

        Self {
            chunk_type: chunk_type,
            data: data,
            length: length,
            crc: crc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
