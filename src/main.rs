extern crate csv;

use std::fs::File;
use std::io;
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

fn filter<R: io::Read>(
    mut rdr: csv::Reader<R>,
    condition: String,
) -> Result<Vec<csv::StringRecord>, Box<dyn std::error::Error>> {
    let mut rows: Vec<csv::StringRecord> = vec![];
    for result in rdr.records() {
        let record = result?;
        let col = f32::from_str(&record[1])?; // magic number one: col_index from condition
        if col > 12.0 {
            // ^ magic number two: value from condition, also operation
            rows.push(record);
        }
    }
    Ok(rows)
}

#[derive(Debug, PartialEq)]
enum Check {
    GreaterThan(f32),
    SmallerThan(f32),
    GreaterThanOrEqual(f32),
    SmallerThanOrEqual(f32),
}

fn get_condition_parts<R: io::Read + std::fmt::Debug>(
    rdr: csv::Reader<R>,
    condition: String,
) -> (usize, Check) {
    println!("{:?} {}", rdr, condition);
    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    match opt.input {
        Some(input) => {
            let file = File::open(input)?;
            let filtered = filter(csv::Reader::from_reader(file), "id>12".to_string())?;
            println!("{:?}", filtered);
            Ok(())
        }
        None => {
            let filtered = filter(csv::Reader::from_reader(io::stdin()), "id>12".to_string())?;
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
        let filtered = filter(rdr, "id>12".to_string()).unwrap();

        assert_eq!(filtered, vec![csv::StringRecord::from(vec!["foo", "42"])])
    }

    #[test]
    fn test_get_condition_parts() {
        let csv = "name,id\nmoo,12\nfoo,42";
        let rdr = csv::Reader::from_reader(csv.as_bytes());
        let parts = get_condition_parts(rdr, "id>12".to_string());
        assert_eq!(parts.0, 1);
        assert_eq!(parts.1, Check::GreaterThan(12.0));
    }
}
