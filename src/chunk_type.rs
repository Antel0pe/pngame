use std::fmt;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub struct ChunkType {
    pub name: [u8; 4]
}


impl ChunkType {
    pub fn bytes(&self) -> [u8; 4]{
        self.name
    }

    pub fn is_valid(&self) -> bool{
        for byte in self.name.into_iter(){
            if !byte.is_ascii_alphabetic(){
                return false;
            }
        }

        return self.is_reserved_bit_valid();
    }

    pub fn is_critical(&self) -> bool{
        self.name[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool{
        self.name[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool{
        println!("{:?}", self.name[2].is_ascii_uppercase());
        self.name[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool{
        self.name[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType{
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error>{
        let chunk_type: ChunkType = ChunkType{ name: value };
        if chunk_type.is_valid() {
            return Ok(chunk_type);
        } else {
            return Err("Not a valid type");
        }
    }
}

impl FromStr for ChunkType{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        if s.len() == 4{
            let chunk_type: ChunkType = ChunkType { name: s.as_bytes()[0..4].try_into().unwrap() };
            return Ok(chunk_type);
        }

        return Err("String is not 4 characters long. ");
    }
}

impl Display for ChunkType{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        write!(f, "{}", std::str::from_utf8(&self.name).unwrap())
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

        // let chunk = ChunkType::from_str("Ru1t");
        // assert!(chunk.is_err());
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
