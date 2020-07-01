extern crate csv;

use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
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
    value: f32,
) -> Result<csv::StringRecord, Box<dyn std::error::Error>> {
    println!("{:?}", rdr.headers()?);
    for result in rdr.records() {
        let record = result?;
        let col = f32::from_str(&record[1])?;
        println!("col: {}", col);
        println!("{:?}", record);
    }
    Ok(csv::StringRecord::from(vec!["foo", "42"]))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    match opt.input {
        Some(input) => {
            let file = File::open(input)?;
            filter(csv::Reader::from_reader(file), 12.0)?;
            Ok(())
        }
        None => {
            filter(csv::Reader::from_reader(std::io::stdin()), 12.0)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_larger_than() {
        let csv = "name,id
            moo,12
            foo,42";
        let rdr = csv::Reader::from_reader(csv.as_bytes());
        let filtered = filter(rdr, 12.0).unwrap();

        assert_eq!(filtered, csv::StringRecord::from(vec!["foo", "42"]))
    }
}
