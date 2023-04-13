use std::str::FromStr;
use std::convert::TryFrom;
use std::fmt;

#[derive(PartialEq,Debug,Eq)]
pub struct ChunkType {
    chunk: [u8;4]
}

impl TryFrom<[u8;4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8;4]) -> Result<Self, Self::Error>{

        for val in value.iter() {
            if !(65..90).contains(val) & !(97..122).contains(val){
                return Err("")
            }
        }
        Ok(ChunkType{chunk:value})
    }
}

impl FromStr for ChunkType{
    type Err =&'static str;

    fn from_str(s:&str) -> Result<Self,Self::Err>{
        let bytes = s.as_bytes();

        let b_iter = bytes.iter();

        for val in b_iter {
            if !(65..90).contains(val) & !(97..122).contains(val){
                return Err("ChunkType invalid");
            }
        }

        Ok(ChunkType{chunk:[bytes[0],bytes[1],bytes[2],bytes[3]]})
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = std::str::from_utf8(&self.chunk);
        write!(f,"{}",text.unwrap())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8;4] {
        self.chunk
    }

    fn is_critical(&self) -> bool {
        self.chunk[0].is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.chunk[1].is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.chunk[2].is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        self.chunk[3].is_ascii_lowercase()
    }

    pub fn is_valid(&self) -> bool {
        if self.is_reserved_bit_valid(){
            for val in self.chunk.iter() {
                if !(65..90).contains(val) & !(97..122).contains(val){
                    return false
                }
            }
            true
        } else {
            false
        }
    }

}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::convert::TryFrom;
//     use std::str::FromStr;

//     #[test]
//     pub fn test_chunk_type_from_bytes() {
//         let expected = [82, 117, 83, 116];
//         let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

//         assert_eq!(expected, actual.bytes());
//     }

//     #[test]
//     pub fn test_chunk_type_from_str() {
//         let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
//         let actual = ChunkType::from_str("RuSt").unwrap();
//         assert_eq!(expected, actual);
//     }

//     #[test]
//     pub fn test_chunk_type_is_critical() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(chunk.is_critical());
//     }

//     #[test]
//     pub fn test_chunk_type_is_not_critical() {
//         let chunk = ChunkType::from_str("ruSt").unwrap();
//         assert!(!chunk.is_critical());
//     }

//     #[test]
//     pub fn test_chunk_type_is_public() {
//         let chunk = ChunkType::from_str("RUSt").unwrap();
//         assert!(chunk.is_public());
//     }

//     #[test]
//     pub fn test_chunk_type_is_not_public() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(!chunk.is_public());
//     }

//     #[test]
//     pub fn test_chunk_type_is_reserved_bit_valid() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(chunk.is_reserved_bit_valid());
//     }

//     #[test]
//     pub fn test_chunk_type_is_reserved_bit_invalid() {
//         let chunk = ChunkType::from_str("Rust").unwrap();
//         assert!(!chunk.is_reserved_bit_valid());
//     }

//     #[test]
//     pub fn test_chunk_type_is_safe_to_copy() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(chunk.is_safe_to_copy());
//     }

//     #[test]
//     pub fn test_chunk_type_is_unsafe_to_copy() {
//         let chunk = ChunkType::from_str("RuST").unwrap();
//         assert!(!chunk.is_safe_to_copy());
//     }

//     #[test]
//     pub fn test_valid_chunk_is_valid() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(chunk.is_valid());
//     }

//     #[test]
//     pub fn test_invalid_chunk_is_valid() {
//         let chunk = ChunkType::from_str("Rust").unwrap();
//         assert!(!chunk.is_valid());

//         let chunk = ChunkType::from_str("Ru1t");
//         assert!(chunk.is_err());
//     }

//     #[test]
//     pub fn test_chunk_type_string() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert_eq!(&chunk.to_string(), "RuSt");
//     }

//     #[test]
//     pub fn test_chunk_type_trait_impls() {
//         let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
//         let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
//         let _chunk_string = format!("{}", chunk_type_1);
//         let _are_chunks_equal = chunk_type_1 == chunk_type_2;
//     }
// }