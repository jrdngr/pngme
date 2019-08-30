use std::fs;
use std::io::Write;

use structopt::StructOpt;

mod args;
mod error;
mod png;
mod encode;
mod decode;
mod remove;

pub use crate::error::{Error, Result};

use crate::args::PngMeArgs;
use crate::png::{Png, Chunk, ChunkType};

fn main() -> Result<()> {
    let args = PngMeArgs::from_args();

    if let PngMeArgs::Encode{file, chunk, message, .. } = args {
        let mut bytes = fs::read(file).unwrap();
        let mut png = Png::from_bytes(&bytes).unwrap();
        println!("{}", png);

        let chunk_type = ChunkType::from_str(&chunk).unwrap();
        let data = message.into_bytes();

        png.insert_chunk(Chunk::new(chunk_type, data));

        println!("{}", png);

        let mut result_file = fs::File::create("out.png").unwrap();
        result_file.write_all(&png.as_bytes()).unwrap();
    }

    Ok(())
}
