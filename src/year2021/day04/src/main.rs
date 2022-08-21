use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    part1();
}

fn part1() {
    let file = File::open("src/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    // read the first line and check how many columns there are
    let mut numbers = String::new();
    let _ = reader.read_line(&mut numbers);

    // iterate through the file
    for line in reader.lines() {}
}

fn part2() {}