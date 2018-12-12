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
use std::string::ToString;
use std::io::BufRead;
use core::fmt;
use std::collections::hash_map::Values;
use itertools::Itertools;

type Input = Vec<String>;
type Output = (usize, usize);

fn solve(input: Input) -> Result<Output, Error> {
    let inp_cloned = input.clone();
    let result = day_6_a(input)?;
    let result_b: usize = day_6_b(inp_cloned)?;
    println!("Result for 6.a {}. Result for 6.b {}.", &result, &result_b);
    Ok((result, result_b))
}

#[derive(Debug, Clone)]
struct Location {
    col: usize,
    row: usize,
}


impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.col, self.row)
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.col == other.col && self.row == other.row
    }
}

pub fn day_6_a(input: Input) -> Result<usize, Error> {
    let number_of_coordinates = input.len();
    let locations_of_coordinates = get_location_of_coordinates(input);
    let size_of_matrix = 400 as usize;
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; size_of_matrix]; size_of_matrix];
    let mut list_of_non_allowed = vec![];
    for c in 0..size_of_matrix {
        for r in 0..size_of_matrix {
            let mut distances_to_coordinates: Vec<usize> = vec![];
            for coordinate_input in &locations_of_coordinates {
                // Get manhattan distance from (c,r) to input coordinate:
                let sum = get_manhattan_distance_to_coordinate(c, r, coordinate_input);
                distances_to_coordinates.push(sum);
            }
            // Is there a way in Rust for finding all_max and/ or find all positions?
            let min_value_in_array = &distances_to_coordinates.iter().min().unwrap();
            let indices_with_max: &Vec<&usize> = &distances_to_coordinates.iter().filter(|p| p == min_value_in_array).collect();
            let (index_with_max, _) = &distances_to_coordinates.iter().find_position(|p| p == min_value_in_array).unwrap();
            if indices_with_max.len() == 1 {
                matrix[c][r] = *index_with_max;
            } else {
                matrix[c][r] = size_of_matrix + 1;
            }
            if r == 0 || c == 0 || r == size_of_matrix - 1 || c == size_of_matrix - 1 {
                let cc = index_with_max.clone();
                list_of_non_allowed.push(cc); // These are on the border; non limited areas.
            }
        }
    }

    let mut sums: Vec<usize> = vec![];
    for inn in 0..number_of_coordinates {
        let cloned = matrix.clone();
        let sum_for_coordinate = cloned.into_iter().fold(0, |acc, val: Vec<usize>| {
            let in_row: Vec<usize> = val.into_iter().filter(|index| *index == inn).collect();
            acc + in_row.len()
        });
        if !list_of_non_allowed.contains(&inn) {
            sums.push(sum_for_coordinate);
        }
    }

    sums.sort();

    Ok(*sums.last().unwrap())
}

fn get_location_of_coordinates(input: Vec<String>) -> Vec<Location> {
    let mut locations_of_coordinates: Vec<Location> = vec![];
    for inp in input {
        let splitted: Vec<&str> = inp.split(", ").collect();
        let col = splitted.get(0).unwrap().parse::<usize>().unwrap();
        let row = splitted.get(1).unwrap().parse::<usize>().unwrap();
        locations_of_coordinates.push(Location { col, row });
    }
    locations_of_coordinates
}

pub fn day_6_b(input: Input) -> Result<usize, Error> {

    let locations_of_coordinates = get_location_of_coordinates(input);
    let size_of_matrix = 400 as usize;
    let mut allowed = vec![];
    for c in 0..size_of_matrix {
        for r in 0..size_of_matrix {
            let mut distances_to_coordinates: Vec<usize> = vec![];
            for coordinate_input in &locations_of_coordinates {
                // Get manhattan distance from (c,r) to input coordinate:
                let sum = get_manhattan_distance_to_coordinate(c, r, coordinate_input);
                distances_to_coordinates.push(sum);
            }
            let sum_distance_to_coordinates: usize = distances_to_coordinates.iter().sum();

            if sum_distance_to_coordinates < 10000 {
                allowed.push((c, r));
            }
        }
    }
    Ok(allowed.len())
}

fn get_manhattan_distance_to_coordinate(c: usize, r: usize, input: &Location) -> usize {
    let diff_col: usize = match input.col > c {
        true => input.col - c,
        _ => c - input.col
    };
    let diff_row: usize = match input.row > r {
        true => input.row - r,
        _ => r - input.row
    };
    let sum: usize = diff_col + diff_row;
    sum
}

fn run() -> Result<(), Error> {
    let input: Vec<String> = read_line_by_line("day6/data/in6.txt")?;
    let _output = solve(input)?;
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
