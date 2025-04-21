mod args;
mod chunk;
mod chunk_type;
use std::str::FromStr;

use crate::chunk_type::{ChunkType};
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let result = ChunkType::try_from([73, 72, 68, 82]);
    // let result = ChunkType::from_str("Ru1t");

    // println!("{:?}", "Rust".as_bytes());
    
    match result {
        Ok(val) => println!("Result is: {}", val),
        Err(e) => println!("Error: {}", e)
    }

    Ok(())
}
