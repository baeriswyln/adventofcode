pub struct Calories {
    data: Vec<usize>,
}

impl crate::Advent for Calories {
    fn new(data: &str) -> Self {
        let mut data = data
            .split("\n\n")
            .map(|l| l
                .lines()
                .map(|e| e.parse::<usize>().unwrap())
                .sum()
            )
            .collect::<Vec<usize>>();
        data.sort_unstable();
        data.reverse();
        Calories {
            data
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 {
        1
    }

    fn part1(&self) -> usize {
        *self.data.get(0).unwrap()
    }

    fn part2(&self) -> usize {
        let mut sum = 0;

        for i in 0..3 {
            sum += *self.data.get(i).unwrap()
        }

        sum
    }
}