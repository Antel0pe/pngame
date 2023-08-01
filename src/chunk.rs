use std::{fmt::Display, io::{BufReader, Read}, string::FromUtf8Error};

use crate::{chunk_type::ChunkType};
use crc::Crc;

#[derive(Debug)]
pub struct Chunk{
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk{
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Chunk, &'static str> {
        let mut reader = BufReader::new(value);

        let mut length_buffer: [u8; 4] = [0, 0, 0, 0];
        let _ = reader.read_exact(&mut length_buffer);

        let mut chunk_type_buffer: [u8; 4] = [0, 0, 0, 0];
        let _ = reader.read_exact(&mut chunk_type_buffer);

        let mut chunk_data_buffer = vec![0; u32::from_be_bytes(length_buffer).try_into().unwrap()];
        let _ = reader.read_exact(&mut chunk_data_buffer);

        let mut crc_buffer: [u8; 4] = [0, 0, 0, 0];
        let _ = reader.read_exact(&mut crc_buffer);

        let crc_value = u32::from_be_bytes(crc_buffer);
        if crc_value != Chunk::get_chunk_checksum(&chunk_type_buffer, &chunk_data_buffer){
            return Err("Given checksum is incorrect.");
        }


        Ok(Chunk{ 
            length: u32::from_be_bytes(length_buffer),
            chunk_type: ChunkType { name: chunk_type_buffer },
            chunk_data: chunk_data_buffer,
            crc: u32::from_be_bytes(crc_buffer),
        })
    }
}

impl Display for Chunk{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length: {}, Chunk Type: {}, Chunk Data: {:?}, CRC: {}", self.length, self.chunk_type, self.chunk_data, self.crc)
    }
}

impl Chunk{
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk{
        Chunk{
            length: data.len().try_into().unwrap(),
            crc: Chunk::get_chunk_checksum(&chunk_type.name, &data),
            chunk_type: chunk_type,
            chunk_data: data,
        }
    }

    pub fn get_chunk_checksum(chunk_type: &[u8; 4], chunk_data: &Vec<u8>) -> u32{
        let crc: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut crc_data = chunk_type.to_vec();
        crc_data.extend(chunk_data.clone());

        crc.checksum(&crc_data)
    }

    pub fn length(&self) -> u32{
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType{
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8]{
        &self.chunk_data
    }

    pub fn crc(&self) -> u32{
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error>{
        String::from_utf8(self.chunk_data.clone())
    }

    pub fn as_bytes(&self) -> Vec<u8>{
        let mut chunk_bytes: Vec<u8> = Vec::new();

        chunk_bytes.extend(self.length().to_be_bytes());
        chunk_bytes.extend(self.chunk_type.name);
        chunk_bytes.extend(self.chunk_data.clone());
        chunk_bytes.extend(self.crc.to_be_bytes());

        chunk_bytes
    }

}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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
        println!("{}", chunk);
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

