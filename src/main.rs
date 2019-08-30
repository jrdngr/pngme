use structopt::StructOpt;

mod args;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let args = args::PngMeArgs::from_args();
    dbg!(args);

    Ok(())
}
