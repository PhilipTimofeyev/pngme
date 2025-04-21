use core::fmt;
use std::fmt::Error;
use std::str::FromStr;
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    pub chunk: [u8; 4]
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.chunk
    }

    fn is_critical(&self) -> bool {
        let first_byte = self.chunk.iter().nth(0).unwrap();
        let char = char::from(*first_byte);

        char == char.to_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        let second_byte = self.chunk.iter().nth(1).unwrap();
        let char = char::from(*second_byte);

        char == char.to_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        let third_byte = self.chunk.iter().nth(2).unwrap();
        let char = char::from(*third_byte);

        char == char.to_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        let fourth_byte = self.chunk.iter().nth(3).unwrap();
        let char = char::from(*fourth_byte);

        char == char.to_ascii_lowercase()
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() &&
        self.is_safe_to_copy()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(chunk: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType { chunk };

        // if chunk_type.is_reserved_bit_valid() && chunk_type.is_safe_to_copy() {
        //     return Ok(chunk_type)
        // } else {
        //     return Err(Error)
        // }

        Ok(chunk_type)
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid_ascii_alphabet = s.chars().all(|c| c.is_ascii_alphabetic());
        let chunk_arr: [u8; 4] = s.as_bytes().try_into().unwrap();
        let chunk = ChunkType { chunk: chunk_arr};

        valid_ascii_alphabet.then(|| chunk).ok_or(Error)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = str::from_utf8(&self.chunk).unwrap().to_string();
        write!(f, "{}", result)
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