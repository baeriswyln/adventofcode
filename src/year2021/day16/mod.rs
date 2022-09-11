use priority_queue::PriorityQueue;

pub struct PacketDecoder {
    field: Vec<Vec<usize>>,
}

impl crate::Advent for PacketDecoder {
    fn new(data: &str) -> Self {
        PacketDecoder {
            field: data
                .lines()
                .map(|line| line
                    .chars()
                    .map(|c| ((c as usize) - 48))
                    .collect())
                .collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 16 }

    fn part1(&self) -> usize {
        0
    }

    fn part2(&self) -> usize {
        0
    }
}