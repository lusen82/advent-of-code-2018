#![allow(dead_code, unused_imports)]

use failure::bail;
use failure::Error;
use rayon::prelude::*;
use strum_macros::EnumString;

use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::{io, process};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

type Input = String;
type Output = i32;

fn solve() -> Result<Output, Error> {
    day_5_a_pop_push()?;
    day_5_b()?;
    Ok(0)
}

pub fn day_5_a_pop_push() -> Result<(), Error> {
    let input: String = parse_input_file("day5/data/in5.txt")?;//"dabAcCaCBAcCcaDA";
   // let input = "dabAcCaCBAcCcaDA";
    let chars: Vec<char> = input.chars().into_iter().collect();

    let mut result: Vec<char> =  Vec::new();
    result.push('?');
    for c in &chars  {
        update_result(&mut result, c);
    }
    println!("Numbers remaining answer 5.a. : {}", &result.len() - 1);
    Ok(())
}

pub fn day_5_b() -> Result<(), Error> {
    let input: String = parse_input_file("day5/data/in5.txt")?;//"dabAcCaCBAcCcaDA";
    //let input = "dabAcCaCBAcCcaDA";
    let chars: Vec<char> = input.chars().into_iter().collect();
    let vec = &chars.clone();

    let unique: HashSet<&char> = HashSet::from_iter(vec);
    let mut remaining = 1000000;
    for u in unique.into_iter() {
        let this_iter_input = chars.clone();
        let limited_chars: Vec<char> = this_iter_input.into_iter().filter(|p| !p.eq_ignore_ascii_case(&u)).collect();
        let mut result: Vec<char> =  Vec::new();
        result.push('?');
        for c in &limited_chars  {
            update_result(&mut result, c);
        }
        if &result.len() - 1 < remaining {
                             remaining = &result.len() - 1;
        println!("Numbers remaining for char {} is : {}", &u, (&result.len() - 1));
        }

    }
    Ok(())
}


fn update_result(result: &mut Vec<char>, c: &char) -> () {
    let first_char = result.pop().unwrap();
    if (first_char.is_uppercase() && c.is_uppercase()) ||
        (first_char.is_lowercase() && c.is_lowercase()) ||
        !first_char.eq_ignore_ascii_case(&c) {
        result.push(first_char);
        result.push(*c);
    }
}

fn run() -> Result<(), Error> {
    let _output = solve()?;
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

fn parse_input_file(file_name: &'static str) -> Result<Input, Error> {
    let mut f = File::open(file_name).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return Ok(contents);
    // `file` goes out of scope, and the "hello.txt" file gets closed
}



