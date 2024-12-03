use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct SafetyLevels {
    levels: Vec<Vec<u32>>,
}

impl SafetyLevels {
    pub fn parse(path: &Path) -> SafetyLevels {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        
        let mut levels = Vec::new();
        
        for line in reader.lines() {
            let line = line.expect("");
            let mut parts = line.split_whitespace();
            
            let mut level: Vec<u32> = Vec::new();
            
            for part in parts {
                let digit: u32 = part.parse().unwrap();
                level.push(digit);
            }
            
            levels.push(level);
        }
        
        SafetyLevels {
            levels
        }
    }
    
    pub fn safety_count(&self) -> u32 {
        let mut safe_count = 0;

        for level in &self.levels {
            if SafetyLevels::is_safe(level) {
                safe_count += 1;
                continue;
            }

            let mut found = false;

            for i in 0..level.len() {
                let mut modified_level = level.clone();
                modified_level.remove(i);
                if SafetyLevels::is_safe(&modified_level) {
                    safe_count += 1;
                    found = true;
                    break;
                }
            }

            if found {
                continue;
            }
        }

        safe_count
    }
    
    fn is_safe(level: &Vec<u32>) -> bool {
        if level.len() < 2 {
            return true;
        }

        let is_increasing = *level.last().unwrap() as i32 - level[0] as i32 > 0;

        for pair in level.windows(2) {
            let diff = pair[1] as i32 - pair[0] as i32;

            if is_increasing {
                if !(diff > 0 && diff < 4) {
                    return false;
                }
            } else {
                if !(diff < 0 && diff > -4) {
                    return false;
                }
            }
        }

        true 
    }
}