use std::fs;
use std::path::{Path, PathBuf};
use days::day1::LocationsLists;
use days::day2::SafetyLevels;
use days::day3::Muls;

fn main() {
    // let day1_path = PathBuf::from("./days/src/day1/input");
    // let canon = fs::canonicalize(&day1_path).unwrap();
    // let ll = LocationsLists::parse(Path::new(&canon));
    // println!("{}", ll.get_difference());
    // println!("{}", ll.get_similarity());
    
    
    // let day2_path = PathBuf::from("./days/src/day2/input");
    // let canon = fs::canonicalize(&day2_path).unwrap();
    // let levels = SafetyLevels::parse(Path::new(&canon));
    // println!("{}", levels.safety_count());

    let day3_path = PathBuf::from("./days/src/day3/input");
    let canon = fs::canonicalize(&day3_path).unwrap();
    let sum = Muls::run2_optimized(Path::new(&canon));
    println!("{}", sum);    
}
