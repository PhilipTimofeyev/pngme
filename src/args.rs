use std::path::PathBuf;
use clap::{Subcommand, Args};


#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    // Print(PrintArgs),
}
#[derive(Args, Debug)]
pub struct EncodeArgs {
    pub filepath: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub filepath: PathBuf,
    pub chunk_type: String
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub filepath: PathBuf,
    pub chunk_type: String
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    // Write me!
}
