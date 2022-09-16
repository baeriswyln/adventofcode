enum Number {
    Snailfish {
        left: Number,
        right: Number
    },
    Regular {
        number: u8
    }
}

impl Number {
    fn reduce(&self) {

    }
}

pub struct Snailfish {
}

impl crate::Advent for Snailfish {
    fn new(data: &str) -> Self {
        Snailfish {
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 18 }

    fn part1(&self) -> usize {
        0
    }

    fn part2(&self) -> usize {
        0
    }
}