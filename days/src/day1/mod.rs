use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct LocationsLists {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl LocationsLists {
    pub fn parse(path: &Path) -> LocationsLists {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        
        let mut left_list = Vec::new();
        let mut right_list = Vec::new();
        
        for line in reader.lines()  {
            let line = line.expect("");
            let line_clone = line.clone();
            let mut parts = line_clone.split_whitespace();
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            
            let left_digit: u32 = left.parse().unwrap();
            let right_digit: u32 = right.parse().unwrap();
            
            match left_list.binary_search(&left_digit) {
                Ok(pos) => left_list.insert(pos, left_digit),
                Err(pos) => left_list.insert(pos, left_digit),
            }
            
            match right_list.binary_search(&right_digit) {
                Ok(pos) => right_list.insert(pos, right_digit),
                Err(pos) => right_list.insert(pos, right_digit)
            }
        }

        LocationsLists {
            left : left_list,
            right : right_list,
        }
    }
    
    pub fn get_difference(&self) -> u32 {
        self.left.iter().zip(self.right.iter()).map(|(x, y)| x.abs_diff(*y)).sum()
    }
    
    pub fn get_similarity(&self) -> u32 {
        let mut similarity = 0;
        
        let mut previous_left  = 0;
        let mut previous_count = 0;
        
        for left in self.left.iter() {
            if *left == previous_left {
                similarity += previous_left * previous_count;
            }
            
            let mut count = 0;
            for right in self.right.iter() {
                if right == left {
                    count += 1;
                }
                
                if right != left && count > 0 {
                    break;
                }
            }
            
            similarity += count * left;
            previous_left = *left;
            previous_count = count;
        }
        
        similarity
    }
}