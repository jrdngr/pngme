use std::fs;

use structopt::StructOpt;

mod args;
mod error;
mod png;
mod encode;
mod decode;
mod remove;

pub use crate::error::{Error, Result};

use crate::args::PngMeArgs;
use crate::png::Png;

fn main() -> Result<()> {
    let args = PngMeArgs::from_args();

    if let PngMeArgs::Encode{file, .. } = args {
        let mut bytes = fs::read(file).unwrap();
        let png = Png::from_bytes(&bytes).unwrap();
        println!("{}", png);
    }

    Ok(())
}
