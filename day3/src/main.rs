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
type Output = (usize, usize);

fn solve(inp: Input) -> Result<Output, Error> {
    let cloned_input = inp.clone();
    let a = day_3_a(inp)?;
    let b = day_3_b(cloned_input)?;
    Ok((a, b))
}


struct Matrix {
    cols: usize,
    rows: usize,
    data: Vec<i32>
}

impl Matrix {
    fn new(cols: usize, rows: usize) -> Matrix {
        Matrix { cols, rows, data: vec![0; cols * rows] }
    }

    fn update(cols: usize, rows: usize, data: Vec<i32>) -> Matrix {
        Matrix { cols, rows, data }
    }
}
use regex::Regex;

pub fn day_3_a(inp: Input) -> Result<usize, Error> {
    println!("Starting..");
    let dimension = 1100;
    let mut m = Matrix::new(dimension, dimension);
    let re = Regex::new(r"[^0-9]+")?;
    for i in inp {
        // col, row, size
        let num_split = re.replace_all(i.as_str(), ";");
        let nums: Vec<&str> = num_split.split(';').collect();

        let col_int_length = nums.get(4).unwrap().parse::<i32>()?;
        let row_int_length = nums.get(5).unwrap().parse::<i32>()?;
        let s_col_int = nums.get(2).unwrap().parse::<i32>()?;
        let s_row_int = nums.get(3).unwrap().parse::<i32>()?;

        for c in 0..col_int_length {
            for r in 0..row_int_length {
                let cc = s_col_int + c;
                let rr = s_row_int + r;
                m = update_matrix_for_position(cc as usize, rr as usize, m);

            }
        }
    }
    let two_or_more = m.data.into_iter().filter(|p | *p > 1).count();
    println!("***************");
    println!("2 or more: {}", two_or_more);
    Ok(two_or_more)
}

pub fn day_3_b(inp: Input) -> Result<usize, Error> {
    println!("Starting..");

    let dimension = 1000;
    let mut m = Matrix::new(dimension, dimension);
    let mut pos_col_row: Vec<(i32, i32, i32,i32, i32)> = vec![];
    let re = Regex::new(r"[^0-9]+").unwrap();
    for i in inp {
        let num_split = re.replace_all(i.as_str(), ";");
        let nums: Vec<&str> = num_split.split(';').collect();
        let id = nums.get(1).unwrap().parse::<i32>()?;
        let col_int_length = nums.get(4).unwrap().parse::<i32>()?;
        let row_int_length = nums.get(5).unwrap().parse::<i32>()?;
        let s_col_int = nums.get(2).unwrap().parse::<i32>()?;
        let s_row_int = nums.get(3).unwrap().parse::<i32>()?;
        pos_col_row.push((id.clone(), s_col_int.clone(), s_row_int.clone(), col_int_length.clone(), row_int_length.clone()));
        for c in 0..col_int_length {
            for r in 0..row_int_length {
                let cc = s_col_int + c;
                let rr = s_row_int + r;
                m = update_matrix_for_position(cc as usize, rr as usize, m);
            }
        }
    }

    for (pos, s_col_int, s_row_int, col_int_length,row_int_length) in pos_col_row {
        let mut was_zero = true;
        for c in 0..col_int_length {
            for r in 0..row_int_length {
                let cc = s_col_int + c;
                let rr = s_row_int + r;
                was_zero = was_zero && check_if_empty_matrix(cc as usize, rr as usize, &m);
            }
        }
        if was_zero {
            println!("Id {} is not hitting others.", &pos);
        }
    }


    let two_or_more = m.data.into_iter().filter(|p | *p > 1).count();

    println!("***************");
    println!("2 or more: {}", two_or_more);
    Ok(two_or_more)
}

fn check_if_empty_matrix(col: usize, row: usize, m: &Matrix) -> bool
{
    let data: &Vec<i32> = &m.data;
    let vector_number  = col % m.cols + (row * m.cols);
    let current_val: &i32 = &data.get(vector_number).unwrap();
    let i = "1".parse::<i32>().unwrap();
    return current_val.eq(&i);
}

fn update_matrix_for_position(col: usize, row: usize, m: Matrix) -> Matrix
{
    let data: Vec<i32> = m.data;

    let vector_number  = col % m.cols + (row * m.cols);

    let mut data_vec = data.clone();

    let current_val = data_vec.remove(vector_number);
    data_vec.insert(vector_number, current_val + 1);

    return Matrix::update(m.cols, m.rows, data_vec);
}

fn run() -> Result<(), Error> {
    let inp: Vec<String> = read_line_by_line("day3/data/in3.txt")?;
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
    fn test_1() {
        println!("Starting test 3:1.");
        let input: Vec<&'static str> = vec! {
            "#1 @ 1,3: 4x4",
            "#2 @ 3,1: 4x4",
            "#3 @ 5,5: 2x2"};

        let as_str: Vec<String> = input.into_iter().map(|s| {
            println!("{}", s.to_string());
            s.to_string()
        }).collect();
        println!("{:?}", as_str);
        if let Ok((output_a, output_b)) = super::solve(as_str)
            {
                assert_eq!(output_a, 8, "Correct checksum answer is 4.");
                // assert_eq!(output_b, "fgij", "Correct checksum answer is fgij.");
            }
    }
}
