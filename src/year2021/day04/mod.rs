struct Board {
    numbers: [[u32; 5]; 5]
}

struct Bingo {
    random_numbers: Vec<u32>,
    boards: Vec<Board>
}

pub struct GiantSquid {
    data: Vec<Bingo>
}

impl crate::Advent for GiantSquid {
    fn new(data: &str) -> Self  {
        todo!()
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 4 }

    fn part1(&self) -> usize {
        0
    }

    fn part2(&self) -> usize {
        0
    }
}