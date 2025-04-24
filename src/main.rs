mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use args::PngMeArgs;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: PngMeArgs,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        PngMeArgs::Encode(args) => commands::encode(args),
        PngMeArgs::Decode(args) => commands::decode(args),
        PngMeArgs::Remove(args) => commands::remove(args),
        PngMeArgs::Print(args) => commands::print_chunks(args),
    };

    if let Err(e) = result {
        println!("{}", e)
    }

    Ok(())
}
