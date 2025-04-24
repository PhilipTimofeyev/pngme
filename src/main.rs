mod args;
mod chunk;
mod chunk_type;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use std::str::FromStr;
mod commands;
mod png;
use crate::png::Png;
use clap::{Parser, Subcommand};
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
    command: Command,
    filepath: PathBuf,
    chunk_type: Option<String>,
    message: Option<String>,
    output_file: Option<String>
}

#[derive(Subcommand)]
enum Command {
    Encode,
    Decode,
    Remove,
    Print
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let args = match cli.command {
        Command::Encode => {
            let encode_args = args::EncodeArgs {
                filepath: PathBuf::from(cli.filepath),
                chunk_type: cli.chunk_type.unwrap(),
                message: cli.message.unwrap(),
                output_file: cli.output_file
            };
            args::PngMeArgs::Encode(encode_args)
        },
        Command::Decode => {
            let decode_args = args::DecodeArgs {
                filepath: PathBuf::from(cli.filepath),
                chunk_type: cli.chunk_type.unwrap(),
            };
            args::PngMeArgs::Decode(decode_args)
        }
        Command::Remove => {
            let remove_args = args::RemoveArgs {
                filepath: PathBuf::from(cli.filepath),
                chunk_type: cli.chunk_type.unwrap(),
            };
            args::PngMeArgs::Remove(remove_args)
        },
        _ => {
            let decode_args = args::DecodeArgs {
                filepath: PathBuf::from(cli.filepath),
                chunk_type: cli.chunk_type.unwrap(),
            };
            args::PngMeArgs::Decode(decode_args)
        },
    };

    let b = match args {
        args::PngMeArgs::Encode(args) => {
            let png_file = read_file(&args.filepath)?;
            let mut result = Png::try_from(&png_file[..])?;
            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let chunk = Chunk::new(chunk_type, args.message.as_bytes().into());
            result.append_chunk(chunk);
            fs::write(&args.filepath, result.as_bytes())?;

            // println!("{:?}", result)
        },
        args::PngMeArgs::Decode(args) => {
            let png_file = read_file(&args.filepath)?;
            let mut result = Png::try_from(&png_file[..])?;
            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let result = result.chunk_by_type(args.chunk_type.as_str()).unwrap();

            println!("{:?}", result.data_as_string()?)
        },
        args::PngMeArgs::Remove(args) => {
            let png_file = read_file(&args.filepath)?;
            let mut png = Png::try_from(&png_file[..])?;
            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let result = png.remove_first_chunk(&args.chunk_type);

            fs::write(&args.filepath, png.as_bytes())?;

            println!("{:?}", result)
        },
        args::PngMeArgs::Print(args) => {
            println!("Print")
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