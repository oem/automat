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

fn filter<'a, R: io::Read + 'a>(
    mut rdr: csv::Reader<R>,
    condition: String,
) -> Result<Vec<csv::StringRecord>, Box<dyn std::error::Error>> {
    let mut rows: Vec<csv::StringRecord> = vec![];
    rows.push(rdr.headers()?.clone());
    for result in rdr.records() {
        let record = result?;
        let col = f32::from_str(&record[1])?; // magic number one: col_index from condition
        if col > 12.0 {
            // ^ magic number two: value from condition, also a magical operation
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

fn get_condition_parts(condition: String) -> Option<Check> {
    Some(Check::GreaterThan(12.))
}

fn get_col_index<R: io::Read + std::fmt::Debug>(rdr: csv::Reader<R>) -> Option<usize> {
    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    match opt.input {
        Some(input) => {
            let file = File::open(input)?;
            let filtered = filter(csv::Reader::from_reader(file), "id>12".to_string())?;
            for row in filtered {
                println!("{}", row.iter().collect::<Vec<&str>>().join(","));
            }
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

        assert_eq!(
            filtered,
            vec![
                csv::StringRecord::from(vec!["name", "id"]),
                csv::StringRecord::from(vec!["foo", "42"])
            ]
        )
    }

    #[test]
    fn get_condition_parts_a() {
        let check = get_condition_parts("id>12".to_string());
        assert_eq!(check, Some(Check::GreaterThan(12.0)));
    }

    #[test]
    fn get_condition_parts_b() {
        let check = get_condition_parts("id<=42".to_string());
        assert_eq!(check, Some(Check::SmallerThanOrEqual(42.)));
    }
}
