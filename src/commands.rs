use std::path::PathBuf;

use clap::Clap;

use crate::chunk_type::ChunkType;

#[derive(Clap)]
pub struct CmdOptions {
    #[clap(subcommand)]
    pub sub_command: Commands,
}

#[derive(Clap, Debug)]
pub enum Commands {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(Clap, Debug)]
pub struct Encode {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String,
}

#[derive(Clap, Debug)]
pub struct Decode {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Clap, Debug)]
pub struct Remove {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Clap, Debug)]
pub struct Print {
    pub file_path: PathBuf,
}
