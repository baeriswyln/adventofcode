use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut last_number = 0;
    let mut first = true;
    let mut result = 0;

    for line in reader.lines() {
        let number: u32 = line.unwrap().parse().expect("Wanted a number");

        if !first && last_number < number {
            result += 1;
        }

        first = false;
        last_number = number;
    }

    println!("Result: {}", result);
}
