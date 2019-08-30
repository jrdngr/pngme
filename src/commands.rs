use std::fs;
use std::io::Write;

use crate::error::Result;
use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};
use crate::png::{Png, Chunk, ChunkType};

pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file)?;

    let chunk_type = ChunkType::from_str(&args.chunk)?;
    let data = args.message.into_bytes();

    png.insert_chunk(Chunk::new(chunk_type, data));

    let file_path = match args.out {
        Some(path) => path,
        None => args.file,
    };

    fs::write(&file_path, &png.as_bytes())?;

    println!("Wrote message to: {:?}", &file_path);

    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file)?;

    match png.chunk_by_type(&args.chunk) {
        Some(message_chunk) => {
            let message = std::str::from_utf8(message_chunk.data())?;
            println!("{}", message);
        },
        None => println!("Error: No chunk of type {}", &args.chunk),
    }

    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file)?;
    png.remove_chunk(&args.chunk)?;
    fs::write(&args.file, &png.as_bytes())?;
    println!("Removed message from: {:?}", &args.file);

    Ok(())
}

pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let bytes = fs::read(&args.file)?;
    let png = Png::from_bytes(&bytes)?;
    println!("{}", png);

    Ok(())
}
