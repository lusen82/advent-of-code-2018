#![allow(dead_code, unused_imports)]

use failure::bail;
use failure::Error;
use rayon::prelude::*;
use strum_macros::EnumString;

use std::char;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::BufRead;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;

type Input = Vec<String>;
type Output = i32;


fn solve(inp: Input) -> Result<Output, Error> {
    let cloned_input = inp.clone();
    let out_a = day_1_a(inp)?;
    day_1_b(cloned_input)?;
    Ok(out_a)
}

fn day_1_a(inp: Input) -> Result<i32, Error> {
    println!("Starting day 1 a");
    let sum = inp.iter().fold(0, |acc, val| {
        return acc + val.parse::<i32>().unwrap();
    });
    println!("1.a. answer: {}", sum);
    Ok(sum)
}

fn day_1_b(inp: Input) -> Result<(), Error> {
    let mut heard =  HashSet::new();
    let mut not_found = true;
    let mut mta = 0;
    while not_found {
        for val in inp.iter() {
            let new_val = mta + val.parse::<i32>()?;
            if heard.contains(&new_val) {
                not_found = false;
                println!("1.b. answer: {}", &new_val);
                break;
            }
            heard.insert(new_val.clone());
            mta = new_val;
        };
    }
    Ok(())
}

fn run() -> Result<(), Error> {
    let inp: Input = read_line_by_line("day1/data/in1.txt")?;
    let output = solve(inp)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => process::exit(0),
        Err(error) => {
            for cause in error.iter_causes() {
                eprintln!("{}", cause)
            }
            process::exit(1)
        }
    }
}

fn read_line_by_line(file_name: &'static str) -> Result<Input, Error> {
    println!("Filename {}", file_name);
    let f = File::open(file_name).expect("file not found");
    let lines_read = BufReader::new(&f).lines();

    let all = lines_read.map(|line| {
        if let Ok(lin) = line {
            return lin;
        } else { return "Error".to_string(); }
    }).collect();

    return Ok(all);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        println!("Starting test 1.");
        let input: Vec<&'static str> = vec!{"+1", "+1",  "+1"};

        let as_str: Vec<String> = input.into_iter().map(|s| {
            println!("{}", s.to_string());
            s.to_string()
        }).collect();
        println!("{:?}", as_str);
        let output = super::day_1_a(as_str);
        assert_eq!(output.unwrap(), 3, "Correct answer is 3.");
    }

    #[test]
    fn test_2() {

        println!("Starting test 2.");
        let input: Vec<&'static str> = vec!{"+1", "+1",  "-2"};
        let as_str: Vec<String> = input.into_iter().map(|s| s.to_string()).collect();
        println!("{:?}", as_str);

        let output = super::day_1_a(as_str);
        assert_eq!(output.unwrap(), 0, "Correct answer is 0.");
    }

    #[test]
    fn test_3() {

        println!("Starting test 3.");
        let input: Vec<&'static str> =
            vec!{"-1", "-2",  "-3"};
        let as_str: Vec<String> = input.into_iter().map(|s| s.to_string()).collect();
        println!("{:?}", as_str);

        let output = super::day_1_a(as_str);
        assert_eq!(output.unwrap(), -6, "Correct answer is -6.");
    }
}
