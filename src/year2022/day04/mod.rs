struct Entry {
    l_start: usize,
    l_end: usize,
    r_start: usize,
    r_end: usize,
}

pub struct CampCleanup {
    data: Vec<Entry>,
}

impl crate::Advent for CampCleanup {
    fn new(data: &str) -> Self {
        CampCleanup {
            data: data.lines().map(|l| {
                let mut e = l.split(['-', ',']);
                Entry {
                    l_start: e.next().unwrap().parse().unwrap(),
                    l_end: e.next().unwrap().parse().unwrap(),
                    r_start: e.next().unwrap().parse().unwrap(),
                    r_end: e.next().unwrap().parse().unwrap(),
                }
            }).collect()
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        4
    }

    fn part1(&self) -> usize {
        self.data.iter().map(|e| {
            if e.l_start <= e.r_start && e.l_end >= e.r_end
                || e.l_start >= e.r_start && e.l_end <= e.r_end {
                1
            } else {
                0
            }
        }).sum()
    }

    fn part2(&self) -> usize {
        self.data.iter().map(|e| {
            if e.l_start <= e.r_start && e.l_end >= e.r_start
                || e.r_start <= e.l_start && e.r_end >= e.l_start{
                1
            } else {
                0
            }
        }).sum()
    }
}