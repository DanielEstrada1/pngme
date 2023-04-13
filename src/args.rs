use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic", about="hide messages in png files")]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
#[derive(StructOpt, Debug)]
#[structopt(name = "encode", about="insert message into png")]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
#[structopt(name="decode",about="decode a specific chunktype")]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}


#[derive(StructOpt, Debug)]
#[structopt(name="remove",about="delete a message with the provided chunk type")]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}
#[derive(StructOpt, Debug)]
#[structopt(name="print",about="print chunks")]
pub struct PrintArgs {
    pub file_path: PathBuf,
}