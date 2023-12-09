mod puzzles;

use std::fs;

use puzzles::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => panic!("Not enough arguments"),
        _ => {
            let day = args[1].as_str();
            let data = fs::read_to_string(format!("data/{}.txt", day)).unwrap();
            match day {
                "day1" => {
                    day1::run(data);
                }
                "day2" => {
                    day2::run(data);
                }
                _ => {
                    panic!("Invalid day");
                }
            }
        }
    }
}
