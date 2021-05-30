use std::{convert::TryFrom, fmt::Display, fmt, str::FromStr};

// Png consists of a PNG signature followed by chunks

// Png files are a list of "chunks"
// - Each chunk has a type, represented by as a 4 character string

// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    chunk_name: [u8; 4],
}

// TryFrom does simple and safe type conversions.
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let mut chunk_type = ChunkType { chunk_name: [0; 4] };

        for (index, v) in value.iter().enumerate() {
            if v.is_ascii() {
                chunk_type.chunk_name[index] =  *v;
            }
        }

        Ok(chunk_type)
    }
}

// FromStr does parsing of a value through a string
impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunk_type = ChunkType { chunk_name: [0; 4] };

        for (index, character) in s.chars().enumerate() {
            if character.is_ascii_lowercase() || character.is_ascii_uppercase() {
                chunk_type.chunk_name[index] = character as u8;
            } else {
                return Err("Could not parse");
            }
        }

        Ok(chunk_type)
    }
}

// Display formats value using a given formatter
impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.chunk_name).unwrap())
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.chunk_name
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && self.chunk_name[0].is_ascii() &&
                                        self.chunk_name[1].is_ascii() &&
                                        self.chunk_name[1].is_ascii() &&
                                        self.chunk_name[3].is_ascii()
    }

    fn is_critical(&self) -> bool {
        // chunks that are not strictly necessary to the display the content of the file
        // is a "ancillary" chunk
        // chunks that are necessary to the display the contents of the file is a "critical" chunk
        self.chunk_name[0].is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        // public chunk is one that is part of the PNG specification
        // private chunk is our own defined chunk for our own purpose
        self.chunk_name[1].is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        // must be 0 in files conforming to the 1.2 version of the PNG spec
        self.chunk_name[2].is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        // if chunk's safe-to-copy bit is 1, chunk may be copied to a modifed PNG file
        // if chunk's safe-to-copy bit is 0, the chunk depend on the image data
        self.chunk_name[3].is_ascii_lowercase()
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