use std::collections::HashMap;

pub struct TreacheryOfWhales {
    data: Vec<usize>,
}

impl crate::Advent for TreacheryOfWhales {
    fn new(data: &str) -> Self {
        TreacheryOfWhales {
            data: data
                .split(",")
                .flat_map(|line| line.parse::<usize>())
                .collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 7 }

    fn part1(&self) -> usize {
        let mut costs = HashMap::new();
        let min = *self.data.iter().min().unwrap();
        let max = *self.data.iter().max().unwrap();

        for &init_pos in &self.data {
            for target_pos in min..=max {
                costs.entry(target_pos)
                    .and_modify(|v| *v += init_pos.abs_diff(target_pos))
                    .or_insert(init_pos.abs_diff(target_pos));
            }
        }

        *costs.values().min().unwrap()
    }

    fn part2(&self) -> usize {
        let mut costs = HashMap::new();
        let min = *self.data.iter().min().unwrap();
        let max = *self.data.iter().max().unwrap();

        for &init_pos in &self.data {
            for target_pos in min..=max {
                costs.entry(target_pos)
                    .and_modify(|v| *v += compute_cost(init_pos, target_pos))
                    .or_insert(compute_cost(init_pos, target_pos));
            }
        }

        *costs.values().min().unwrap()
    }
}

fn compute_cost(init : usize, target : usize) -> usize {
    let dif = init.abs_diff(target);
    (0..=dif).sum()
}