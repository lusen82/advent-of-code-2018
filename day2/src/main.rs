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
type Output = (i32, String);

fn solve(inp: Input) -> Result<Output, Error> {
    let cloned_input = inp.clone();
    let a = day_2_a(inp)?;
    let b = day_2_b(cloned_input)?;
    Ok((a, b))
}

pub fn day_2_a(inp: Input) -> Result<i32, Error> {

    let twice = get_match_for_value(&inp, 2);
    let three = get_match_for_value(&inp, 3);
    println!("Answer for 2.a. is the multiplier {}", twice * three);
    Ok(twice * three)
}

fn get_match_for_value(inp: &Vec<String>, value: usize) -> i32 {
    let as_many_times_as_val = inp.iter().fold(0, |acc, val| {
        let val_clone = val.clone();
        let booleans: Vec<bool> = val.chars().map(|f| {
            let nr = val_clone.chars().filter(|p| p.eq(&f)).count();
            return nr == value;
        }).collect();

        if booleans.into_iter().find(|p| *p).is_some() {
            return acc + 1;
        }
        return acc;
    });
    as_many_times_as_val
}

pub fn day_2_b(inp: Input) -> Result<String, Error> {
     let mut matching_except_one: (&str, &str) = ("", "");
    for input in &inp {
        for others in &inp {
            let mut number_of_misses = 0;
            for (c1, c2) in input.chars().into_iter().zip(others.chars().into_iter()) {
                if c1 != c2 {
                    number_of_misses += 1;
                }
            }
            if number_of_misses == 1 {
                println!("Number of misses was one for {} and {}", input, others);
                matching_except_one = (input, others);
                break;
            }
        }
    }


    let mut inc_common : String = "".to_string();
    for iter in 0..matching_except_one.0.len() {
        if matching_except_one.0.chars().nth(iter).unwrap() == matching_except_one.1.chars().nth(iter).unwrap() {
           inc_common.push(matching_except_one.0.chars().nth(iter).unwrap());
        }
    }

    Ok(inc_common)
}


fn run() -> Result<(), Error> {

    let inp: Input = read_line_by_line("day2/data/in2.txt")?;
    let _output = solve(inp)?;
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
        println!("Starting test 2:1.");
        let input: Vec<&'static str> = vec!{
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab",
            "abcdef"};

        let as_str: Vec<String> = input.into_iter().map(|s| {
            println!("{}", s.to_string());
            s.to_string()
        }).collect();
        println!("{:?}", as_str);
        if let Ok((output_a, output_b)) = super::solve(as_str)
        {

        assert_eq!(output_a, 12, "Correct checksum answer is 12.");
        //assert_eq!(output_b, "fgij", "Correct checksum answer is fgij.");
        }
    }
    #[test]
    fn test_2() {
        println!("Starting test 2:2.");
        let input: Vec<&'static str> = vec!{
            "abcde",
            "fghij",
            "klmno",
            "pqrst",
            "fguij",
            "axcye",
            "wvxyz"};

        let as_str: Vec<String> = input.into_iter().map(|s| {
            println!("{}", s.to_string());
            s.to_string()
        }).collect();
        println!("{:?}", as_str);
        if let Ok((output_a, output_b)) = super::solve(as_str)
        {

        //assert_eq!(output_a, 12, "Correct checksum answer is 12.");
        assert_eq!(output_b, "fgij", "Correct checksum answer is fgij.");
        }
    }


}
