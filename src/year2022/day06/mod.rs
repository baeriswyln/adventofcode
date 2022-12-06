use std::collections::HashMap;

pub struct TuningTrouble {
    data: String,
}

impl crate::Advent for TuningTrouble {
    fn new(data: &str) -> Self {
        TuningTrouble {
            data: data.to_string()
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        6
    }

    fn part1(&self) -> usize {
        find_first_unique_string(&self.data, 4)
    }

    fn part2(&self) -> usize {
        find_first_unique_string(&self.data, 14)
    }
}

fn find_first_unique_string(input: &String, len: usize) -> usize {
    // initialize buffer with first letter
    let mut buffer = vec![input.as_bytes()[0] as char;len];
    let mut buffer_pos = 0;

    let mut index = 0;

    for c in input.chars() {
        index += 1;

        buffer[buffer_pos] = c;
        buffer_pos = (buffer_pos + 1) % len;

        let mut map = HashMap::new();

        for &i in &buffer {
            map.insert(i, true);
        }

        if map.len() == len {
            break;
        }
    }

    index
}