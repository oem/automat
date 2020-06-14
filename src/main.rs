use std::{
    io::{self, BufRead},
    path,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    cmd: String,
    #[structopt(parse(from_os_str))]
    file: path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    println!("exploratory data analysis via the command line");
    println!("running {} on file {:?}", args.cmd, args.file);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
    Ok(())
}
