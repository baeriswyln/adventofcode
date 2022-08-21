pub struct SonarSweep {
    data: Vec<u32>,
}

impl crate::Advent for SonarSweep {
    fn new(data: &str) -> Self {
        SonarSweep {
            data: data.lines().flat_map(|l| l.parse()).collect(),
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 {
        1
    }

    fn part1(&self) -> usize {
        let mut last_number = 0;
        let mut first = true;
        let mut result = 0;

        for number in self.data.iter() {
            if !first && last_number < *number {
                result += 1;
            }

            first = false;
            last_number = *number;
        }

        result
    }

    fn part2(&self) -> usize {
        let mut index = 0;
        let mut numbers = [0.0; 3];
        let mut first = true;
        let mut result = 0;
        let mut last_number = 0.0;

        for line in self.data.iter() {
            numbers[index % 3] = *line as f64;

            if index >= 2 {
                let mut total = 0.0;
                for number in numbers {
                    total += number;
                }

                if first {
                    first = false;
                } else if total / 3.0 > last_number {
                    result += 1;
                }

                last_number = total / 3.0;
            }

            index += 1;
        }

        result
    }
}