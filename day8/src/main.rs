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
use regex::Captures;
use linked_hash_set::LinkedHashSet;
use core::slice;
use failure::err_msg;
type Input = String;
type Output = (u32, u32);

fn solve(inp:String) -> Result<Output, Error> {
    let inp_b = inp.clone();
    let sum_a = day_8_a(inp)?;
    let sum_b = day_8_b(inp_b)?;
    Ok((sum_a, sum_b))
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
    value: Vec<u32>,
    branch: Vec<Node>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

fn new_node(children: Vec<Node>, value: Vec<u32>) -> Node {
    Node {value, branch: children}
}

fn new_leaf(elem: Vec<u32>) -> Node {
    Node {value: elem, branch: vec![]}
}

const RADIX: u32 = 10;


pub fn day_8_a(inp:String) -> Result<(u32), Error> {
    let mut trimmed: Vec<&str> = inp.split_whitespace().collect();
    let input: &mut Vec<&str> = trimmed.as_mut();
    input.reverse();
    let top_node: Node = get_next_node(input)?;
    let sum = get_sum_meta_data_a(&top_node);
    println!("SUM IS FOR A {}", &sum);
    Ok(sum)

}

pub fn day_8_b(inp:String) -> Result<(u32), Error> {
    let mut trimmed: Vec<&str> = inp.split_whitespace().collect();
    let input: &mut Vec<&str> = trimmed.as_mut();
    input.reverse();
    let top_node: Node = get_next_node(input)?;
    let sum = get_sum_meta_data_b(&top_node);
    println!("SUM IS FOR B {}", &sum);
    Ok(sum)
}

fn get_sum_meta_data_a(top_node: &Node) -> u32 {
    let sum_this: u32 = top_node.value.iter().sum();
    let mut add = 0;
    for ch in &top_node.branch {
        add += get_sum_meta_data_a(&ch);
    }
    add + sum_this
}

fn get_sum_meta_data_b(top_node: &Node) -> u32 {
    let amount_of_children = top_node.branch.len();
    let sum_this: u32 = match amount_of_children {
        0 =>  top_node.value.iter().sum(),
        _ => 0
    };
    let mut add = 0;
    for md in &top_node.value {
        if let Some(ch) = top_node.branch.get(*md as usize - 1)  {
            add += get_sum_meta_data_b(&ch);
        }
    }
    add + sum_this
}


fn get_next_node(input: &mut Vec<&str>) -> Result<Node, Error>
{
    let mut children = vec![];
    let nr_of_children = input.pop().ok_or_else(|| err_msg("empty input"))?.parse::<u32>()?;
    let nr_of_meta = input.pop().ok_or_else(|| err_msg("empty input"))?.parse::<u32>()?;
    for _ in 0..nr_of_children {
        let updated_node = get_next_node(input)?;
        children.push(updated_node);

    }
    let mut meta_data = vec![];
    for _ in 0..nr_of_meta{
        let n = input.pop().ok_or_else(|| err_msg("empty input"))?.parse::<u32>()?;
        meta_data.push(n);
    }
    Ok(Node {branch: children, value: meta_data})
}


fn run() -> Result<(), Error> {
    let input: String = parse_input_file("day8/data/in8.txt")?;
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
fn parse_input_file(file_name: &'static str) -> Result<Input, Error> {
    let mut f = File::open(file_name).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return Ok(contents);
    // `file` goes out of scope, and the "hello.txt" file gets closed
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_btree_creation() {
        super::new_leaf(vec![10]);

        let node: super::Node = super::new_node(vec![super::new_leaf(vec![15]), super::new_leaf(vec![20])], vec![30]);
        super::new_node(vec![node.clone(), super::new_leaf(vec![30])], vec![20]);
        println!("Node: {}", node);
        assert_eq!(node, node.clone());
    }

    // 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2

    #[test]
    fn test_1() {
        println!("Starting test 1.");
        let input: &str =  "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

        println!("Input test 1: {}", input);

        let output = super::solve(input.to_string()).unwrap();
        assert_eq!(output.0, 138, "Correct answer is 138.");
        assert_eq!(output.1, 66, "Correct answer is 66.");
    }
}
