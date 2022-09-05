pub struct PassagePathing {
    data: Vec<Vec<u8>>,
}

impl crate::Advent for PassagePathing {
    fn new(data: &str) -> Self {
        PassagePathing {
            data: data
                .lines()
                .map(|line|
                    line.chars()
                        .map(|c| (c as u8) - 48)
                        .collect())
                .collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 11 }

    fn part1(&self) -> usize {
        0
    }

    fn part2(&self) -> usize {
        0
    }
}