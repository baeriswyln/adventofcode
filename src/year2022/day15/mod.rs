pub struct BeaconExclusionZone {
    data: Vec<String>,
}

impl crate::Advent for BeaconExclusionZone {
    fn new(data: &str) -> Self {
        BeaconExclusionZone {
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