use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> anyhow::Result<()> {
    let mut png = Png::from_file(&args.file)?;

    let chunk_type = ChunkType::from_str(&args.chunk)?;
    let data = args.message.into_bytes();

    png.append_chunk(Chunk::new(chunk_type, data));

    let file_path = match args.out {
        Some(path) => path,
        None => args.file,
    };

    fs::write(&file_path, &png.as_bytes())?;

    println!("Wrote message to: {:?}", &file_path);

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> anyhow::Result<()> {
    let png = Png::from_file(&args.file)?;

    match png.chunk_by_type(&args.chunk) {
        Some(message_chunk) => {
            let message = std::str::from_utf8(message_chunk.data())?;
            println!("{}", message);
        }
        None => println!("Error: No chunk of type {}", &args.chunk),
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> anyhow::Result<()> {
    let mut png = Png::from_file(&args.file)?;
    png.remove_chunk(&args.chunk)?;
    fs::write(&args.file, &png.as_bytes())?;
    println!("Removed message from: {:?}", &args.file);

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.file)?;
    let png = Png::try_from(bytes.as_ref())?;
    println!("{}", png);

    Ok(())
}
