use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme")]
pub enum PngMeArgs {
    #[structopt(name = "encode")]
    Encode {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
        chunk: String,
        message: String,
        #[structopt(short = "o", long = "out", parse(from_os_str))]
        out: Option<PathBuf>,
    },
    #[structopt(name = "decode")]
    Decode {
        file: PathBuf,
        chunk: String,
    },
    #[structopt(name = "remove")]
    Remove {
        file: PathBuf,
        chunk: String,
    },
}
