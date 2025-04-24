use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};
use crate::{Result, chunk};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let png_file = read_file(&args.filepath)?;
    let mut result = Png::try_from(&png_file[..])?;

    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().into());

    result.append_chunk(chunk);

    fs::write(&args.filepath, result.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png_file = read_file(&args.filepath)?;
    let result = Png::try_from(&png_file[..])?;
    let decoded_message =
        result
            .chunk_by_type(args.chunk_type.as_str())
            .ok_or(chunk::ChunkError::NotFound(
                "Chunktype not found.".to_string(),
            ))?;

    println!("{}", decoded_message.data_as_string()?);

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let png_file = read_file(&args.filepath)?;
    let mut png = Png::try_from(&png_file[..])?;
    let result = png.remove_first_chunk(&args.chunk_type)?;

    fs::write(&args.filepath, png.as_bytes())?;

    println!(
        "\nRemoved chunk: {} \nMessage: {}",
        result.chunk_type,
        result.data_as_string()?
    );

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    todo!()
}

fn read_file(filepath: &PathBuf) -> Result<Vec<u8>> {
    let mut f = File::open(filepath)?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;

    Ok(data)
}
