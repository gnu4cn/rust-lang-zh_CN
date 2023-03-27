#![allow(warnings)]
// 
// this is to disable warnings.
// Comment it to enable warnings.
//
use std::error::Error;
use std::fs;
use data_structures::Config;
//
// 以下两种写法，也是可以的
//
// use self::data_structures::Config;
// use crate::data_structures::Config;

#[cfg(test)]
mod tests;
pub mod data_structures;

pub fn run(
    config: Config
) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results: Vec<&str> = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println! ("{line}");
    }

    Ok(())
}

pub fn search<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
