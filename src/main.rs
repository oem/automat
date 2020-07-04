extern crate csv;

use regex::Regex;
use std::error::Error;
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
        condition: String,
    },
}

fn filter<'a, R: io::Read + 'a>(
    mut rdr: csv::Reader<R>,
    condition: String,
) -> Result<Vec<csv::StringRecord>, Box<dyn Error>> {
    let mut rows: Vec<csv::StringRecord> = vec![];
    if let Ok(Some(c)) = get_condition_parts(condition) {
        println!("identified condition: {:?}", c);
    }
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

fn get_condition_parts(condition: String) -> Result<Option<Check>, Box<dyn Error>> {
    let re = Regex::new(r"(>=|<|>|<=|==)(\d+)$")?;
    let mut operator = "".to_string();
    let mut value = 0.;
    for cap in re.captures_iter(&condition) {
        operator = cap[1].to_string();
        value = f32::from_str(&cap[2])?; // magic number one: col_index from condition
    }
    match operator.as_str() {
        ">=" => Ok(Some(Check::GreaterThanOrEqual(value))),
        ">" => Ok(Some(Check::GreaterThan(value))),
        "<=" => Ok(Some(Check::SmallerThanOrEqual(value))),
        "<" => Ok(Some(Check::SmallerThan(value))),
        _ => Ok(None),
    }
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
    fn get_condition_parts_a() -> Result<(), Box<dyn std::error::Error>> {
        let check = get_condition_parts("id>12".to_string())?;
        assert_eq!(check, Some(Check::GreaterThan(12.0)));
        Ok(())
    }

    #[test]
    fn get_condition_parts_b() -> Result<(), Box<dyn std::error::Error>> {
        let check = get_condition_parts("id<=42".to_string())?;
        assert_eq!(check, Some(Check::SmallerThanOrEqual(42.)));
        Ok(())
    }
}
