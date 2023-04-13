
use std::convert::TryFrom;
use std::fs::{self, File};
use std::str::FromStr;
use std::path::PathBuf;

use crate::{Error, Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {

    let file = fs::read(args.file_path)?;
    let mut png = Png::try_from(file.as_slice())?;
    let bytes = args.message.as_bytes().to_vec();
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;

    let chunk = Chunk::new(chunk_type,bytes);
    
    png.append_chunk(chunk);

    let output_file_path = if let Some(output_file_name) = args.output_file {
        output_file_name
    } else {
        PathBuf::from_str("edited.png").unwrap()
    };

    fs::write(output_file_path,png.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let file = fs::read(args.file_path)?;
    let png = Png::try_from(file.as_slice())?;

    if let Some(chunk) = png.chunk_by_type(&args.chunk_type.as_str()){
        println!("{}",chunk);
    } else {
        eprintln!("Chunk Type: {} not found", args.chunk_type);
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let file = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(file.as_slice())?;

    png.remove_chunk(&args.chunk_type)?;
    fs::write(args.file_path,png.as_bytes().as_slice())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let file = fs::read(args.file_path)?;
    let png = Png::try_from(file.as_slice())?;
    let png_string = format!("{}", png);
    println!("{}",png_string);
    Ok(())
}