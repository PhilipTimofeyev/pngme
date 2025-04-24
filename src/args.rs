use std::path::PathBuf;

pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
#[derive(Debug)]
pub struct EncodeArgs {
    pub filepath: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>
}

pub struct DecodeArgs {
    pub filepath: PathBuf,
    pub chunk_type: String
}

pub struct RemoveArgs {
    pub filepath: PathBuf,
    pub chunk_type: String
}

pub struct PrintArgs {
    // Write me!
}
