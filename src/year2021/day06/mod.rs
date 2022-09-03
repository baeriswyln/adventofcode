use std::collections::HashMap;

pub struct Lanternfish {
    data: Vec<u8>,
}

impl crate::Advent for Lanternfish {
    fn new(data: &str) -> Self {
        Lanternfish {
            data: data
                .split(",")
                .flat_map(|line| line.parse::<u8>())
                .collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 6 }

    fn part1(&self) -> usize {
        simulate(&self.data, 80)
    }

    fn part2(&self) -> usize {
        simulate(&self.data, 256)
    }
}

fn simulate(data : &Vec<u8>, rounds : usize) -> usize{
    let mut values = HashMap::new();

    for &val in data {
        values
            .entry(val)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    for _ in 0..rounds {
        let mut new_values = HashMap::new();

        for (i, n) in values {
            if i == 0 {
                new_values.insert(8, n);
                new_values
                    .entry(6)
                    .and_modify(|v| *v += n)
                    .or_insert(n);
            } else {
                new_values
                    .entry(i - 1)
                    .and_modify(|v| *v += n)
                    .or_insert(n);
            }
        }
        values = new_values;
    }

    let mut result = 0;
    for (_, n) in values {
        result += n;
    }
    result
}