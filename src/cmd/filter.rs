use regex::Regex;
use std::error::Error;
use std::fmt;
use std::io;
use std::str::FromStr;

pub fn filter<'a, R: io::Read + fmt::Debug + 'a>(
    mut rdr: csv::Reader<R>,
    condition: &str,
) -> Result<Vec<csv::StringRecord>, Box<dyn Error>> {
    let index = get_col_index(&mut rdr, condition)?;
    let check = get_condition_parts(condition)?;
    let headers = rdr.headers()?.clone();
    let mut rows: Vec<csv::StringRecord> = rdr
        .records()
        .flatten()
        .filter(|row| {
            if let Ok(col) = f32::from_str(&row[index]) {
                return check.compare(col);
            }
            false
        })
        .collect();
    rows.insert(0, headers);

    Ok(rows)
}

#[derive(Debug, PartialEq)]
enum Check {
    GreaterThan(f32),
    SmallerThan(f32),
    GreaterThanOrEqual(f32),
    SmallerThanOrEqual(f32),
    Equal(f32),
}

impl Check {
    fn compare(&self, other: f32) -> bool {
        match &self {
            Self::GreaterThan(n) => other > *n,
            Self::GreaterThanOrEqual(n) => other >= *n,
            Self::SmallerThan(n) => other < *n,
            Self::SmallerThanOrEqual(n) => other <= *n,
            Self::Equal(n) => {
                let error_margin = f32::EPSILON;
                (*n - other).abs() < error_margin
            }
        }
    }
}

#[derive(Debug)]
struct ParseConditionError {
    details: String,
}

impl ParseConditionError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseConditionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseConditionError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn get_condition_parts(condition: &str) -> Result<Check, Box<dyn Error>> {
    let re = Regex::new(r"(>=|<|>|<=|==)(-*\d+)$")?;
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
        "==" => Ok(Check::Equal(value)),
        a => Err(Box::new(ParseConditionError::new(
            format!("Unknown operator {}", a).as_str(),
        ))),
    }
}

fn get_col_index<R: io::Read + fmt::Debug>(
    rdr: &mut csv::Reader<R>,
    condition: &str,
) -> Result<usize, Box<dyn Error>> {
    let re = Regex::new(r"^(.+?)(>=|<|>|<=|==)")?;
    let mut col_name = "".to_string();
    for cap in re.captures_iter(condition) {
        col_name = cap[1].to_string();
    }

    let index = rdr
        .headers()?
        .iter()
        .position(|r| r == col_name)
        .ok_or(format!("Column {} not found in the headers", col_name))?;
    Ok(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_larger_than() -> Result<(), Box<dyn Error>> {
        let csv = "name,id\nmoo,12\nfoo,42";
        let rdr = csv::Reader::from_reader(csv.as_bytes());
        let filtered = filter(rdr, "id>12")?;

        assert_eq!(
            filtered,
            vec![
                csv::StringRecord::from(vec!["name", "id"]),
                csv::StringRecord::from(vec!["foo", "42"])
            ]
        );
        Ok(())
    }

    #[test]
    fn test_filter_na_values() -> Result<(), Box<dyn Error>> {
        let csv = "name,id\nmoo,12\ngoo,\nfoo,42";
        let rdr = csv::Reader::from_reader(csv.as_bytes());
        let filtered = filter(rdr, "id>12")?;

        assert_eq!(
            filtered,
            vec![
                csv::StringRecord::from(vec!["name", "id"]),
                csv::StringRecord::from(vec!["foo", "42"])
            ]
        );
        Ok(())
    }

    #[test]
    fn test_filter_negative() {
        let csv = "name,id\nmoo,12\nfoo,42";
        let rdr = csv::Reader::from_reader(csv.as_bytes());
        let filtered = filter(rdr, "id>-12").unwrap();
        assert_eq!(
            filtered,
            vec![
                csv::StringRecord::from(vec!["name", "id"]),
                csv::StringRecord::from(vec!["moo", "12"]),
                csv::StringRecord::from(vec!["foo", "42"])
            ]
        );
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

    #[test]
    fn test_compare_equal() {
        assert_eq!(Check::Equal(4.).compare(4.), true);
        assert_eq!(Check::Equal(42.).compare(12.), false);
    }

    #[test]
    fn test_get_col_index_a() -> Result<(), Box<dyn Error>> {
        let csv = "name,id\nmoo,12\nfoo,42";
        let rdr = &mut csv::Reader::from_reader(csv.as_bytes());
        assert_eq!(get_col_index(rdr, "id>12")?, 1);
        assert_eq!(get_col_index(rdr, "name>12")?, 0);
        Ok(())
    }

    #[test]
    fn test_get_col_index_no_header_match() {
        let csv = "name,id\nmoo,12\nfoo,42";
        let rdr = &mut csv::Reader::from_reader(csv.as_bytes());
        assert_eq!(get_col_index(rdr, "weird>=42").is_err(), true);
        assert_eq!(get_col_index(rdr, "something<=42").is_err(), true);
    }

    #[test]
    fn test_get_col_index_no_headers() {
        let csv = "moo,12\nfoo,42";
        let rdr = &mut csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv.as_bytes());
        assert_eq!(get_col_index(rdr, "weird>=42").is_err(), true);
    }
}
