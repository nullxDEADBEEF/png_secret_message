use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use crc::crc32;

use crate::chunk_type::ChunkType;
use crate::{Error, Result};

#[derive(Debug)]
pub struct Chunk {
    // number of bytes in the chunk's data
    // can be any length, up to 4 bytes
    length: u32,
    typee: ChunkType,
    data: Vec<u8>,
    // CRC calculated on preeceding bytes in the chunk (chunk type and data)
    // this is always present even if there is no data.
    // used to verify each chunk for corrupted data
    crc: u32,
}

impl Chunk {
    fn new(length: u32, typee: ChunkType, data: Vec<u8>, crc: u32) -> Self {
        Self {
            length,
            typee,
            data,
            crc,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.typee
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let string = String::from_utf8(self.data.clone());
        match string {
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .cloned()
            .chain(self.chunk_type().name.iter().cloned())
            .chain(self.data().iter().cloned())
            .chain(self.crc().to_be_bytes().iter().cloned())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8; 4] = [0; 4];

        // get first 4 bytes to determine length
        reader.read_exact(&mut buffer)?;
        let data_length = u32::from_be_bytes(buffer);

        // get next 4 bytes to determine chunk type
        reader.read_exact(&mut buffer).unwrap();
        let chunk_type = ChunkType::try_from(buffer)?;

        let mut data_buffer = vec![0; data_length as usize];
        reader.read_exact(&mut data_buffer)?;
        let chunk_data = data_buffer;

        reader.read_exact(&mut buffer)?;
        let received_crc = u32::from_be_bytes(buffer);

        let crc = crc32::checksum_ieee(&[&chunk_type.name, chunk_data.as_slice()].concat());
        let chunk = Chunk::new(data_length, chunk_type, chunk_data, crc);

        if chunk.crc() == received_crc {
            Ok(chunk)
        } else {
            Err("Invalid chunk".into())
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Length: {}\nType: {}\nData bytes: {}\nCRC: {}\n",
            self.length(),
            self.chunk_type(),
            self.data.len(),
            self.crc()
        )
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
