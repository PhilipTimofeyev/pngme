use std::path::PathBuf;
use clap::{Parser, Subcommand};

// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// struct Cli {
//     command: String,

//     // #[arg(short, long, value_name = "FILE")]
//     // config: String,
// }

// pub fn get_args() {
//     let cli = Cli::parse();

//     // You can check the value provided by positional arguments, or option arguments
//     // if let Some(name) = cli.command {
//         println!("Value for name: {}", cli.command);
//     // }
// }


pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
#[derive(Debug)]
pub struct EncodeArgs {
    pub file: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>
}

pub struct DecodeArgs {
    // Write me!
}

pub struct RemoveArgs {
    // Write me!
}

pub struct PrintArgs {
    // Write me!
}
