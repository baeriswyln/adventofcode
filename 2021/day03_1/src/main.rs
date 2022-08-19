use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    // read the first line and check how many columns there are
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);

    // create an array of integers with size = columns
    let mut counters = vec![0; first_line.chars().count()-2];

    let mut number_lines = 0;

    // iterate through the file
    for line in reader.lines() {
        // count the number of lines
        number_lines += 1;

        // add the value of each column to the array
        for (i, c) in line.expect("no val").chars().enumerate() {
            counters[i] += if c == '1' { 1 } else { 0 };
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for val in counters {
        gamma <<= 1;
        epsilon <<= 1;
        gamma |= if val < number_lines / 2 { 1 } else { 0 };
        epsilon |= if val > number_lines / 2 { 1 } else { 0 };
    }

    println!("gamma {}", gamma);
    println!("epsilon {}", epsilon);
    println!("Result {}", gamma * epsilon);
}
