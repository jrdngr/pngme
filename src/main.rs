use structopt::StructOpt;

mod args;
mod error;
mod commands;
pub mod png;

pub use crate::error::{Error, Result};

use crate::args::PngMeArgs;
use crate::commands::{encode, decode, remove, print_chunks};

fn main() -> Result<()> {
    let args = PngMeArgs::from_args();

    match args {
        PngMeArgs::Encode(encode_args) => encode(encode_args),
        PngMeArgs::Decode(decode_args) => decode(decode_args),
        PngMeArgs::Remove(remove_args) => remove(remove_args),
        PngMeArgs::PrintChunks(print_args) => print_chunks(print_args),
    }
}
