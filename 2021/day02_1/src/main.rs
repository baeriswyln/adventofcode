use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut depth = 0;
    let mut position = 0;

    for line in reader.lines(){
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let instruction = split.next().expect("missing");
        let number : u32 = split.next().expect("missing").parse().expect("not a number");

        match instruction {
            "forward" => position += number,
            "down" => depth += number,
            "up" => depth -= number,
            _ => ()
        }
    }


    println!("Result: {}", depth * position);
}
