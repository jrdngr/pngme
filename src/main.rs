use structopt::StructOpt;

mod args;
mod error;
mod png;
mod encode;
mod decode;
mod remove;

use crate::error::Error;

fn main() -> Result<(), Error> {
    let args = args::PngMeArgs::from_args();
    dbg!(args);

    Ok(())
}
