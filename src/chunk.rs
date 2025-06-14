use crate::chunk_type::ChunkType;
use core::fmt;
use crc::{CRC_32_ISO_HDLC, Crc};
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Chunk {
    pub data_length: u32,
    pub chunk_type: ChunkType,
    pub chunk_data: Vec<u8>,
    pub crc: u32,
}

#[derive(Debug)]
pub enum ChunkError {
    InvalidCRC(String),
    NotFound(String),
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkError::InvalidCRC(e) => write!(f, "Chunk Error: {}", e),
            ChunkError::NotFound(e) => write!(f, "Chunk Error: {}", e),
        }
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let data_length = data.len();
        let crc = get_crc(&chunk_type, &data);

        Chunk {
            data_length: data_length.try_into().unwrap(),
            chunk_type,
            chunk_data: data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.data_length
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let length_as_u8 = u32::to_be_bytes(self.length());
        for byte in length_as_u8 {
            result.push(byte)
        }

        for byte in self.chunk_type.chunk_type {
            result.push(byte);
        }

        for byte in self.data() {
            result.push(*byte);
        }

        let length_as_u8 = u32::to_be_bytes(self.crc());
        for byte in length_as_u8 {
            result.push(byte)
        }

        result
    }
}

impl std::error::Error for ChunkError {}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(chunk_data: &[u8]) -> Result<Self> {
        let length: [u8; 4] = chunk_data[0..=3].try_into()?;
        let length: u32 = u32::from_be_bytes(length);

        let chunk_type: [u8; 4] = chunk_data[4..=7].try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type)?;

        let data_range: usize = (length + 7) as usize;
        let data: Vec<u8> = chunk_data[8..=data_range].try_into()?;

        let crc: [u8; 4] = chunk_data[(data_range + 1)..].try_into()?;
        let crc = u32::from_be_bytes(crc);

        let validated_crc = get_crc(&chunk_type, &data);
        let is_valid_crc = validated_crc == crc;

        let chunk = Chunk {
            data_length: length,
            chunk_type,
            chunk_data: data,
            crc,
        };

        is_valid_crc.then_some(chunk).ok_or({
            let error_message = "Invalid CRC".to_string();
            ChunkError::InvalidCRC(error_message).into()
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

fn get_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
    const PNG_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut digest = PNG_CRC.digest();
    digest.update(chunk_type.chunk_type.as_ref());
    digest.update(data);
    digest.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
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
