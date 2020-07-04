extern crate csv;

use regex::Regex;
use std::error::Error;
use std::fmt;
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
    condition: &str,
) -> Result<Vec<csv::StringRecord>, Box<dyn Error>> {
    let mut rows: Vec<csv::StringRecord> = vec![];
    let check = get_condition_parts(condition)?;
    rows.push(rdr.headers()?.clone());
    for result in rdr.records() {
        let record = result?;
        let col = f32::from_str(&record[1])?; // magic number one: col_index from condition
        if check.compare(col) {
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

impl Check {
    fn compare(&self, other: f32) -> bool {
        match &self {
            Self::GreaterThan(n) => {
                println!("{} > {} = {}", other, *n, other > *n);
                other > *n
            }
            Self::GreaterThanOrEqual(n) => other >= *n,
            Self::SmallerThan(n) => other < *n,
            Self::SmallerThanOrEqual(n) => other <= *n,
        }
    }
}

#[derive(Debug)]
struct ParseOperatorError {
    details: String,
}

impl ParseOperatorError {
    fn new(msg: &str) -> ParseOperatorError {
        ParseOperatorError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseOperatorError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn get_condition_parts(condition: &str) -> Result<Check, Box<dyn Error>> {
    let re = Regex::new(r"(>=|<|>|<=|==)(\d+)$")?;
    let mut operator = "".to_string();
    let mut value = 0.;
    for cap in re.captures_iter(condition) {
        operator = cap[1].to_string();
        value = f32::from_str(&cap[2])?;
    }

    match operator.as_str() {
        ">=" => Ok(Check::GreaterThanOrEqual(value)),
        ">" => Ok(Check::GreaterThan(value)),
        "<=" => Ok(Check::SmallerThanOrEqual(value)),
        "<" => Ok(Check::SmallerThan(value)),
        a @ _ => Err(Box::new(ParseOperatorError::new(
            format!("Unknown operator {}", a).as_str(),
        ))),
    }
}

fn get_col_index<R: io::Read + std::fmt::Debug>(rdr: csv::Reader<R>) -> Option<usize> {
    todo!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let filter_condition = match opt.cmd {
        Command::Filter { condition } => condition,
    };
    match opt.input {
        Some(input) => {
            let file = File::open(input)?;
            let filtered = filter(csv::Reader::from_reader(file), filter_condition.as_str())?;
            for row in filtered {
                println!("{}", row.iter().collect::<Vec<&str>>().join(","));
            }
            Ok(())
        }
        None => {
            let filtered = filter(csv::Reader::from_reader(io::stdin()), "id>12")?;
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
        let filtered = filter(rdr, "id>12").unwrap();

        assert_eq!(
            filtered,
            vec![
                csv::StringRecord::from(vec!["name", "id"]),
                csv::StringRecord::from(vec!["foo", "42"])
            ]
        )
    }

    #[test]
    fn test_get_condition_parts_greater_than() -> Result<(), Box<dyn Error>> {
        let check = get_condition_parts(&"id>12")?;
        assert_eq!(check, Check::GreaterThan(12.0));
        Ok(())
    }

    #[test]
    fn test_get_condition_parts_greater_than_or_equal() -> Result<(), Box<dyn Error>> {
        let check = get_condition_parts(&"id>=14")?;
        assert_eq!(check, Check::GreaterThanOrEqual(14.0));
        Ok(())
    }

    #[test]
    fn test_get_condition_parts_smaller_than_or_equal() -> Result<(), Box<dyn Error>> {
        let check = get_condition_parts("id<=42")?;
        assert_eq!(check, Check::SmallerThanOrEqual(42.));
        Ok(())
    }

    #[test]
    fn get_condition_parts_smaller_than() -> Result<(), Box<dyn Error>> {
        let check = get_condition_parts("id<42")?;
        assert_eq!(check, Check::SmallerThan(42.));
        Ok(())
    }

    #[test]
    fn test_get_condition_parts_unknown_operator() {
        let check = get_condition_parts("id!42");
        assert_eq!(check.is_err(), true);
    }

    #[test]
    fn test_compare_greater_than() {
        assert_eq!(Check::GreaterThan(4.).compare(12.), true);
        assert_eq!(Check::GreaterThan(42.).compare(12.), false);
    }

    #[test]
    fn test_compare_greater_than_or_equal() {
        assert_eq!(Check::GreaterThanOrEqual(4.).compare(4.), true);
        assert_eq!(Check::GreaterThanOrEqual(42.).compare(12.), false);
    }

    #[test]
    fn test_compare_smaller_than() {
        assert_eq!(Check::SmallerThan(4.).compare(4.), false);
        assert_eq!(Check::SmallerThan(42.).compare(12.), true);
        assert_eq!(Check::SmallerThan(2.).compare(12.), false);
    }

    #[test]
    fn test_compare_smaller_than_or_equal() {
        assert_eq!(Check::SmallerThanOrEqual(4.).compare(4.), true);
        assert_eq!(Check::SmallerThanOrEqual(42.).compare(12.), true);
        assert_eq!(Check::SmallerThanOrEqual(2.).compare(12.), false);
    }
}
