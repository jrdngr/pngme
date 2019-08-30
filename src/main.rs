use structopt::StructOpt;

mod args;
mod error;
mod png;
mod encode;
mod decode;
mod remove;
mod utils;

pub use crate::error::{Error, Result};

fn main() -> Result<()> {
    let args = args::PngMeArgs::from_args();
    dbg!(args);

    Ok(())
}
