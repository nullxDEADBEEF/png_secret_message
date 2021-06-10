mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::{env, path::PathBuf, str::FromStr};

use clap::Clap;
use commands::{CmdOptions, Commands};

use crate::{chunk_type::ChunkType};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let cmd_options: CmdOptions = CmdOptions::parse();

    match cmd_options.sub_command {
        Commands::Encode(mut e) => {
            e.file_path = PathBuf::from(args.get(2).expect("Invalid index"));
            e.chunk_type = ChunkType::from_str(args.get(3).expect("invalid index")).expect("invalid chunk type");
            e.message = args.get(4).expect("invalid index").to_string();
            args::encode(e)?
        }
        Commands::Decode(mut d) => {
            d.file_path = PathBuf::from(args.get(2).expect("Invalid index"));
            d.chunk_type = ChunkType::from_str(args.get(3).expect("invalid index")).expect("invalid chunk type");
            args::decode(d)?
        }
        Commands::Remove(mut r) => {
            r.file_path = PathBuf::from(args.get(2).expect("Invalid index"));
            r.chunk_type = ChunkType::from_str(args.get(3).expect("invalid index")).expect("invalid chunk type");
            args::remove(r)?
        }
        Commands::Print(mut p) => {
            p.file_path = PathBuf::from(args.get(2).expect("Invalid index"));
            args::print(p)?
        }
    }

    Ok(())
}
