use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme")]
pub enum PngMeArgs {
    #[structopt(name = "encode")]
    Encode(EncodeArgs),
    #[structopt(name = "decode")]
    Decode(DecodeArgs),
    #[structopt(name = "remove")]
    Remove(RemoveArgs),
    #[structopt(name = "print")]
    PrintChunks(PrintArgs),
}

#[derive(StructOpt, Debug)]
pub struct EncodeArgs {
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
    pub chunk: String,
    pub message: String,
    #[structopt(short = "o", long = "out", parse(from_os_str))]
    pub out: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
pub struct DecodeArgs {
    pub file: PathBuf,
    pub chunk: String,
}

#[derive(StructOpt, Debug)]
pub struct RemoveArgs {
    pub file: PathBuf,
    pub chunk: String,
}

#[derive(StructOpt, Debug)]
pub struct PrintArgs {
    pub file: PathBuf,
}
