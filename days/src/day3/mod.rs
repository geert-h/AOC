use std::cmp::PartialEq;
use std::fs;
use std::path::Path;
use regex::Regex;
use crate::day3::InstructionTime::{Do, Dont, Mul, Unknown};

pub struct Muls;

impl Muls {
    pub fn run(path: &Path) -> u32 {
        let input = fs::read_to_string(path).expect("Failed to read");
        
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();  
        
        let mut sum = 0;
        
        for cap in re.captures_iter(&input) {
            let x_str = &cap[1];
            let y_str = &cap[2];
            
            let x: u32 = x_str.parse().unwrap();
            let y: u32 = y_str.parse().unwrap();
            
            sum += x * y;
        }

        sum
    }
    
    pub fn run2_optimized(path: &Path) -> u32 {
        let input = fs::read_to_string(path).expect("Failed to read");

        let rem_re = Regex::new(r"don't\(\)(?s).*?(?:do\(\)|$)").unwrap();
        
        let res = rem_re.replace_all(&input, "");

        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

        let mut sum = 0;

        for cap in re.captures_iter(&res) {
            let x_str = &cap[1];
            let y_str = &cap[2];

            let x: u32 = x_str.parse().unwrap();
            let y: u32 = y_str.parse().unwrap();

            sum += x * y;
        }

        sum
    }
    
    pub fn run2(path: &Path) -> u32 {
        let input = fs::read_to_string(path).expect("Failed to read");

        let mul_re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        let do_re = Regex::new(r"do\(\)").unwrap();
        let dont_re = Regex::new(r"don't\(\)").unwrap();
        
        let rem_re = Regex::new(r"don't\(\)(?s).*do\(\)").unwrap();
        
        let mut sum = 0;
        let mut enabled = true;
        
        let mut pos = 0;
        
        while pos < input.len() {
            let do_match = do_re.find(&input[pos..]);
            let dont_match = dont_re.find(&input[pos..]);
            let mul_match = mul_re.captures(&input[pos..]);
            

            let mut next_pos = input.len() - pos;
            let mut instruction_type = Unknown;

            if let Some(mat) = do_match {
                if mat.start() < next_pos {
                    next_pos = mat.start();
                    instruction_type = Do;
                }
            }

            if let Some(mat) = dont_match {
                if mat.start() < next_pos {
                    next_pos = mat.start();
                    instruction_type = Dont;
                }
            }

            if let Some(cap) = &mul_match {
                if cap.get(0).unwrap().start() < next_pos {
                    next_pos = cap.get(0).unwrap().start();
                    instruction_type = Mul;
                }
            }

            if instruction_type == Unknown {
                break;
            }

            pos += next_pos;

            match instruction_type {
                Do => {
                    enabled = true;
                    let mat = do_re.find(&input[pos..]).unwrap();
                    pos += mat.end();
                },
                Dont => {
                    enabled = false;
                    let mat = dont_re.find(&input[pos..]).unwrap();
                    pos += mat.end();
                },
                Mul => {
                    let cap = mul_re.captures(&input[pos..]).unwrap();
                    if enabled {
                        let x: u32 = cap[1].parse().unwrap();
                        let y: u32 = cap[2].parse().unwrap();
                        sum += x * y;
                    }
                    pos += cap.get(0).unwrap().end();
                },
                _ => {},
            }
        }
        
        sum
    }
}

#[derive(PartialEq)]
pub enum InstructionTime {
    Unknown,
    Do,
    Dont,
    Mul
}