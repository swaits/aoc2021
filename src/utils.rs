use anyhow::Result;
use std::str::FromStr;

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
