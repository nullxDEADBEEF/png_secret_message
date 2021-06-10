use std::convert::TryFrom;
use std::fs;

use crate::commands::{Encode, Decode, Remove, Print};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::Result;

pub fn encode(e: Encode) -> Result<()> {
    let img_data = fs::read(&e.file_path);
    match img_data {
        Ok(img) => {
            let mut png = Png::try_from(img.as_slice())?;
            png.append_chunk(Chunk::new(e.chunk_type, e.message.as_bytes().to_vec()));
            fs::write(e.file_path, png.as_bytes())?;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

pub fn decode(d: Decode) -> Result<()> {
    let img_data = fs::read(&d.file_path);
    match img_data {
        Ok(img) => {
            let png = Png::try_from(img.as_slice())?;
            let chunk = png.chunk_by_type(&d.chunk_type.to_string());
            println!("Hidden message: {}", chunk.unwrap().data_as_string()?);
        },
        Err(e) => { eprintln!("Error: {}", e)}
    }
    Ok(())
}

pub fn remove(r: Remove) -> Result<()> {
    let img_data = fs::read(&r.file_path);
    match img_data {
        Ok(img) => {
            let mut png = Png::try_from(img.as_slice())?;
            png.remove_chunk(&r.chunk_type.to_string())?;
            fs::write(r.file_path, png.as_bytes())?;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

pub fn print(p: Print) -> Result<()> {
    let img_data = fs::read(&p.file_path);
    match img_data {
        Ok(img) => {
            let png = Png::try_from(img.as_slice())?;
            for chunk in png.chunks() {
                println!("{}", chunk);
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}