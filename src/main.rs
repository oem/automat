extern crate csv;
mod cmd;

use csv::Reader;
use std::error::Error;
use std::io::{self, Write};
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

/// this prevents a panic when output is ended prematurely, e.g. by piping to `head`.
fn swallow_pipe_error(res: Result<(), io::Error>) -> Result<(), io::Error> {
    match res {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(other) => Err(other),
    }
}

fn print_table(filtered: Vec<csv::StringRecord>) -> Result<(), io::Error> {
    let mut stdout = io::BufWriter::new(io::stdout());
    for row in filtered {
        writeln!(&mut stdout, "{}", row.iter().collect::<Vec<&str>>().join(","))?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let filter_condition = match opt.cmd {
        Command::Filter { condition } => condition,
    };

    // remove whitespace
    let filter_condition: String = filter_condition.replace(" ", "");

    match opt.input {
        Some(input) => {
            let filtered = cmd::filter(Reader::from_path(input)?, filter_condition.as_str())?;
            swallow_pipe_error(print_table(filtered))?;
            Ok(())
        }
        None => {
            let filtered =
                cmd::filter(Reader::from_reader(io::stdin()), filter_condition.as_str())?;
            swallow_pipe_error(print_table(filtered))?;
            Ok(())
        }
    }
}
