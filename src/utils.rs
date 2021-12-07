use anyhow::Result;
use std::str::FromStr;

pub fn parse_by_char<T: FromStr>(s: &str, c: char) -> Result<Vec<T>> {
    let mut ret = Vec::new();
    for token in s.split(c) {
        let data = match token.trim().parse::<T>() {
            Ok(d) => d,
            Err(_) => panic!("parse error"),
        };
        ret.push(data);
    }
    Ok(ret)
}
pub fn parse_data<T: FromStr>(s: &str) -> Result<Vec<T>> {
    let mut ret = Vec::new();
    for l in s.lines() {
        let data = match l.parse::<T>() {
            Ok(d) => d,
            Err(_) => panic!("parse error"),
        };
        ret.push(data);
    }
    Ok(ret)
}
