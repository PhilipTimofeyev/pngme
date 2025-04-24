mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use std::str::FromStr;
use crate::png::Png;
use args::PngMeArgs;
use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
 pub struct Cli {
    #[command(subcommand)]
    command: args::PngMeArgs,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        PngMeArgs::Encode(args ) => {
            let png_file = read_file(&args.filepath)?;
            let mut result = Png::try_from(&png_file[..])?;
            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let chunk = Chunk::new(chunk_type, args.message.as_bytes().into());
            result.append_chunk(chunk);
            fs::write(&args.filepath, result.as_bytes())?;
        },
        PngMeArgs::Decode(args ) => {
            let png_file = read_file(&args.filepath)?;
            let mut result = Png::try_from(&png_file[..])?;
            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let result = result.chunk_by_type(args.chunk_type.as_str()).unwrap();

            println!("{:?}", result.data_as_string()?)
        }
        PngMeArgs::Remove(args ) => {
            let png_file = read_file(&args.filepath)?;
            let mut png = Png::try_from(&png_file[..])?;
            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let result = png.remove_first_chunk(&args.chunk_type);

            fs::write(&args.filepath, png.as_bytes())?;
        },
        _ => {
            println!("test")
        },
    };

    Ok(())
}

fn read_file(filepath: &PathBuf) -> Result<Vec<u8>> {
    let mut f = File::open(filepath)?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;

    Ok(data)
}