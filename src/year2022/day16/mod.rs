
pub struct ProboscideaVolcanium {
    data: Vec<String>,
}

impl crate::Advent for ProboscideaVolcanium {
    fn new(data: &str) -> Self {
        ProboscideaVolcanium {
            data: vec![String::new(); data.len()]
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        13
    }

    fn part1(&self) -> usize {
        self.data.len()
    }

    fn part2(&self) -> usize {
        0
    }
}