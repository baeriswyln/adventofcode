use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut index = 0;
    let mut numbers = [0.0; 3];
    let mut first = true;
    let mut result = 0;
    let mut last_number = 0.0;

    for line in reader.lines() {
        numbers[index % 3] = line.unwrap().parse().expect("Wanted a number");

        if index >= 2 {
            let mut total = 0.0;
            for number in numbers {
                total += number;
            }

            if first {
                first = false;
            } else if total / 3.0 > last_number {
                result += 1;
            }

            last_number = total/3.0;
        }

        index += 1;
    }

    println!("Result: {}", result);
}
