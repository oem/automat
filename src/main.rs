extern crate csv;
mod cmd;

use csv::Reader;
use std::error::Error;
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

    // remove whitespace
    let filter_condition: String = filter_condition.replace(" ", "");

    fn print_table(filtered: Vec<csv::StringRecord>) {
        for row in filtered {
            println!("{}", row.iter().collect::<Vec<&str>>().join(","));
        }
    }

    match opt.input {
        Some(input) => {
            let filtered = cmd::filter(Reader::from_path(input)?, filter_condition.as_str())?;
            print_table(filtered);
            Ok(())
        }
        None => {
            let filtered =
                cmd::filter(Reader::from_reader(io::stdin()), filter_condition.as_str())?;
            print_table(filtered);
            Ok(())
        }
    }
}
