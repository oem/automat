extern crate csv;

use std::fs::File;
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
        conditions: Vec<String>,
    },
}

fn filter<R: std::io::Read>(
    mut rdr: csv::Reader<R>,
) -> Result<csv::StringRecord, Box<dyn std::error::Error>> {
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(csv::StringRecord::from(vec!["42", "dog", "56"]))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    match opt.input {
        Some(input) => {
            let file = File::open(input)?;
            filter(csv::Reader::from_reader(file))?;
            Ok(())
        }
        None => {
            filter(csv::Reader::from_reader(std::io::stdin()))?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_larger_than() {
        let csv = "id,moo,points
            12,cow,33
            42,dog,56";
        let rdr = csv::Reader::from_reader(csv.as_bytes());
        let filtered = filter(rdr).unwrap();

        assert_eq!(filtered, csv::StringRecord::from(vec!["42", "dog", "56"]))
    }
}
