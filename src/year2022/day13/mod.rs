enum Value {
    Number(usize),
    List(Vec<Value>),
}

struct Pair {
    left: Value,
    right: Value,
}


pub struct DistressSignal {
    data: Vec<Pair>,
}

impl crate::Advent for DistressSignal {
    fn new(data: &str) -> Self {
        DistressSignal {
            data: vec![]
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        13
    }

    fn part1(&self) -> usize {
        0
    }

    fn part2(&self) -> usize {
        0
    }
}