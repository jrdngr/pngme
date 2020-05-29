use clap::Clap;

mod args;
mod commands;
pub mod png;

use crate::args::PngMeArgs;
use crate::commands::{encode, decode, remove, print_chunks};

fn main() -> anyhow::Result<()> {
    let args = PngMeArgs::parse();

    match args {
        PngMeArgs::Encode(encode_args) => encode(encode_args),
        PngMeArgs::Decode(decode_args) => decode(decode_args),
        PngMeArgs::Remove(remove_args) => remove(remove_args),
        PngMeArgs::Print(print_args) => print_chunks(print_args),
    }
}
