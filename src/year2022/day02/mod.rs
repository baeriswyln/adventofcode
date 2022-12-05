pub struct RockPaperScissor {
    data: Vec<String>,
}

impl crate::Advent for RockPaperScissor {
    fn new(data: &str) -> Self {
        RockPaperScissor {
            data: data.lines().map(|l| l.to_string()).collect()
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        2
    }

    fn part1(&self) -> usize {
        self.data.iter().map(|l| {
            let mut c = l.chars();
            let o = c.nth(0).unwrap();
            let i = c.nth(1).unwrap();
            match i {
                'X' => 1 + match o {
                    'A' => 3,
                    'B' => 0,
                    'C' => 6,
                    _ => unreachable!()
                },
                'Y' => 2 + match o {
                    'A' => 6,
                    'B' => 3,
                    'C' => 0,
                    _ => unreachable!()
                },
                'Z' => 3 + match o {
                    'A' => 0,
                    'B' => 6,
                    'C' => 3,
                    _ => unreachable!()
                },
                _ => unreachable!()
            }
        }).sum()
    }

    fn part2(&self) -> usize {
        self.data.iter().map(|l| {
            let mut c = l.chars();
            let o = c.nth(0).unwrap();
            let i = c.nth(1).unwrap();
            match i {
                'X' => 0 + match o {
                    'A' => 3,
                    'B' => 1,
                    'C' => 2,
                    _ => unreachable!()
                },
                'Y' => 3 + match o {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    _ => unreachable!()
                },
                'Z' => 6 + match o {
                    'A' => 2,
                    'B' => 3,
                    'C' => 1,
                    _ => unreachable!()
                },
                _ => unreachable!()
            }
        }).sum()
    }
}