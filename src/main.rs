use std::{
    io::{self, BufRead},
    path,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    cmd: String,
    conditions: String,
    #[structopt(parse(from_os_str))]
    input: path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    println!("exploratory data analysis via the command line");
    println!(
        "cmd {}, conditions {}, input {:?}",
        args.cmd, args.conditions, args.input
    );

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
    Ok(())
}
