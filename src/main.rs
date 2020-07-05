extern crate csv;
mod cmd;

use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "automat",
    about = "Exploratory data analysis via the command line."
)]
struct Opt {
    /// select subcommand
    #[structopt(subcommand)]
    cmd: Command,

    /// tabular data input, stdin if not present
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// filter columns by condition provided
    Filter {
        #[structopt(required = true)]
        condition: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let filter_condition = match opt.cmd {
        Command::Filter { condition } => condition,
    };
    match opt.input {
        Some(input) => {
            let file = File::open(input)?;
            let filtered = cmd::filter(
                &mut csv::Reader::from_reader(file),
                filter_condition.as_str(),
            )?;
            for row in filtered {
                println!("{}", row.iter().collect::<Vec<&str>>().join(","));
            }
            Ok(())
        }
        None => {
            let filtered = cmd::filter(
                &mut csv::Reader::from_reader(io::stdin()),
                filter_condition.as_str(),
            )?;
            println!("{:?}", filtered);
            Ok(())
        }
    }
}
