struct Sample {
    digits: Vec<String>,
    shown: Vec<String>,
}

pub struct SevenSegment {
    data: Vec<Sample>,
}

impl crate::Advent for SevenSegment {
    fn new(data: &str) -> Self {
        SevenSegment {
            data: data
                .lines()
                .map(|line| {
                    let mut iter = line.split(" | ");
                    Sample {
                        digits: iter.next().unwrap().split_whitespace().map(|s| String::from(s)).collect(),
                        shown: iter.next().unwrap().split_whitespace().map(|s| String::from(s)).collect(),
                    }
                })
                .collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 8 }

    fn part1(&self) -> usize {
        let mut result = 0;
        for sample in &self.data {
            for d in &sample.shown {
                if [2, 3, 4, 7].contains(&(d.len() as i32)) {
                    result += 1;
                }
            }
        }
        result
    }

    fn part2(&self) -> usize {
        let mut result = 0;
        for sample in &self.data {
            let mut digits = [""; 10];
            // find out numbers with unique number of letters
            for d in &sample.digits {
                match d.len() {
                    2 => digits[1] = d.as_str(),
                    3 => digits[7] = d.as_str(),
                    4 => digits[4] = d.as_str(),
                    7 => digits[8] = d.as_str(),
                    _ => ()
                }
            }

            // find out remaining numbers with length 6
            for d in &sample.digits {
                if d.len() == 6 {
                    // 6 : länge 6 und etwas fehlt von der 1
                    if !digits[1].chars().all(|c| d.contains(c)) { digits[6] = d }
                    // 0 : länge 6 und etwas fehlt von der 4
                    else if !digits[4].chars().all(|c| d.contains(c)) { digits[0] = d }
                    // 9 : länge 6 ansonsten
                    else { digits[9] = d }
                }
            }

            // do length 5 digits lats as they depend on length 6 digits
            for d in &sample.digits {
                if d.len() == 5 {
                    // 3 : länge 5 und hat alles von 1
                    if digits[1].chars().all(|c| d.contains(c)) { digits[3] = d }
                    // 5 : kann in die 9 gepasst werden
                    else if d.chars().all(|c| digits[9].contains(c)) { digits[5] = d }
                    // 2 : letzte möglichkeit
                    else { digits[2] = d }
                }
            }

            // decode the result
            let mut i = 3;
            let mut val = 0;
            for d in &sample.shown {
                val += digits.iter()
                    .position(|&x|
                        x.len() == d.len() && d.chars().all(|c| x.contains(c)))
                    .unwrap() * 10_usize.pow(i);
                i -= 1;
            }
            result += val;
        }
        result
    }
}