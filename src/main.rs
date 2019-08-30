use std::fs;
use std::io::Write;

use structopt::StructOpt;

mod args;
mod error;
mod png;

pub use crate::error::{Error, Result};

use crate::args::PngMeArgs;
use crate::png::{Png, Chunk, ChunkType};

fn main() -> Result<()> {
    let args = PngMeArgs::from_args();

    match args {
        PngMeArgs::Encode{file, chunk, message, .. } => {
            let mut bytes = fs::read(file).unwrap();
            let mut png = Png::from_bytes(&bytes).unwrap();

            let chunk_type = ChunkType::from_str(&chunk).unwrap();
            let data = message.into_bytes();

            png.insert_chunk(Chunk::new(chunk_type, data));

            let file_name = "out.png";
            let mut result_file = fs::File::create(file_name).unwrap();
            result_file.write_all(&png.as_bytes()).unwrap();

            println!("Wrote message to: {}", file_name);
        },
        PngMeArgs::Decode{file, chunk} => {
            let mut bytes = fs::read(file).unwrap();
            let mut png = Png::from_bytes(&bytes).unwrap();

            match png.chunk_by_type(&chunk) {
                Some(message_chunk) => {
                    let message = std::str::from_utf8(message_chunk.data()).unwrap();
                    println!("{}", message);
                },
                None => println!("Error: No chunk of type {}", chunk),
            }
        },
        PngMeArgs::Remove{file, chunk} => {},
    }

    Ok(())
}
