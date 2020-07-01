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
    col_index: usize,
) -> Result<Vec<csv::StringRecord>, Box<dyn std::error::Error>> {
    let mut rows: Vec<csv::StringRecord> = vec![];
    for result in rdr.records() {
        let record = result?;
        let col = f32::from_str(&record[col_index])?;
        if col > value {
            rows.push(record);
        }
    }
    Ok(rows)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    match opt.input {
        Some(input) => {
            let file = File::open(input)?;
            let filtered = filter(csv::Reader::from_reader(file), 12.0, 1)?;
            println!("{:?}", filtered);
            Ok(())
        }
        None => {
            let filtered = filter(csv::Reader::from_reader(std::io::stdin()), 12.0, 1)?;
            println!("{:?}", filtered);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_larger_than() {
        let csv = "name,id\nmoo,12\nfoo,42";
        let rdr = csv::Reader::from_reader(csv.as_bytes());
        let filtered = filter(rdr, 12.0, 1).unwrap();

        assert_eq!(filtered, vec![csv::StringRecord::from(vec!["foo", "42"])])
    }
}
