use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use crc::CRC_32_ISO_HDLC;


use crate::{Error, Result};
use crate::chunk_type::ChunkType;

pub struct Chunk {
    length : u32,
    ct : ChunkType,
    cd : Vec<u8>,
    crc: u32
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Self {
        let bytes = [chunk_type.bytes().to_vec(),chunk_data.clone()].concat();

        let length: u32 = chunk_data.len().try_into().unwrap();

        let crc_generator = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC);

        let crc:u32 = crc::Crc::<u32>::checksum(&crc_generator,&bytes);

        return Chunk { length, ct:chunk_type,cd:chunk_data,crc};
        
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.ct
    }

    pub fn data_as_string(&self) -> Result<String> {
        let x =String::from_utf8(self.cd.clone())?;
        Ok(x)
    }

    pub fn data(&self) -> &[u8] {
        &self.cd
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let data_length: u32 = self.length();
        let chunk_type = self.chunk_type().bytes();
        let message_bytes = self.data();
        let crc : u32 = self.crc();

        let bytes : Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        bytes
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes:&[u8]) -> Result<Self> {
        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8;4] = [0,0,0,0];

        reader.read_exact(&mut buffer)?;
        let data_length = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer)?;
        let ch_ty = ChunkType::try_from(buffer).unwrap();

        let mut d_buf = vec![0;data_length.try_into().unwrap()];

        reader.read_exact(&mut d_buf)?;
        reader.read_exact(&mut buffer)?;
        let crc_byte = u32::from_be_bytes(buffer);

        let chunk_test = Chunk::new(ch_ty,d_buf);

        if chunk_test.crc() == crc_byte {
            return Ok(chunk_test)
        } else {
            return Err("".into())
        }

        // return Ok(Chunk{length:data_length,ct:chTy,cd:dBuf,crc:crc_byte})

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::chunk_type::ChunkType;
//     use std::str::FromStr;

//     fn testing_chunk() -> Chunk {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656334;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();
        
//         Chunk::try_from(chunk_data.as_ref()).unwrap()
//     }

//     #[test]
//     fn test_new_chunk() {
//         let chunk_type = ChunkType::from_str("RuSt").unwrap();
//         let data = "This is where your secret message will be!".as_bytes().to_vec();
//         let chunk = Chunk::new(chunk_type, data);
//         assert_eq!(chunk.length(), 42);
//         assert_eq!(chunk.crc(), 2882656334);
//     }

//     #[test]
//     fn test_chunk_length() {
//         let chunk = testing_chunk();
//         assert_eq!(chunk.length(), 42);
//     }

//     #[test]
//     fn test_chunk_type() {
//         let chunk = testing_chunk();
//         assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
//     }

//     #[test]
//     fn test_chunk_string() {
//         let chunk = testing_chunk();
//         let chunk_string = chunk.data_as_string().unwrap();
//         let expected_chunk_string = String::from("This is where your secret message will be!");
//         assert_eq!(chunk_string, expected_chunk_string);
//     }

//     #[test]
//     fn test_chunk_crc() {
//         let chunk = testing_chunk();
//         assert_eq!(chunk.crc(), 2882656334);
//     }

//     #[test]
//     fn test_valid_chunk_from_bytes() {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656334;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

//         let chunk_string = chunk.data_as_string().unwrap();
//         let expected_chunk_string = String::from("This is where your secret message will be!");

//         assert_eq!(chunk.length(), 42);
//         assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
//         assert_eq!(chunk_string, expected_chunk_string);
//         assert_eq!(chunk.crc(), 2882656334);
//     }

//     #[test]
//     fn test_invalid_chunk_from_bytes() {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656333;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         let chunk = Chunk::try_from(chunk_data.as_ref());

//         assert!(chunk.is_err());
//     }

//     #[test]
//     pub fn test_chunk_trait_impls() {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656334;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();
        
//         let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
//         let _chunk_string = format!("{}", chunk);
//     }
// }