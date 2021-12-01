use anyhow::Result;
use std::{fs, str::FromStr};

pub fn read_data<T: FromStr>(filename: &str) -> Result<Vec<T>> {
    parse_data(&read_file(filename)?)
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

pub fn read_file(filename: &str) -> Result<String> {
    Ok(fs::read_to_string(filename)?)
}
