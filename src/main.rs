use std::{self, path};
use structopt::StructOpt;


#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

fn main() {
    let args = Args::from_args();
    let res = flatten::flatten(&args.path);
    println!("{:?}", res);
}
