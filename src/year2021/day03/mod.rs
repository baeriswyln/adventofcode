type Bin = Vec<u8>;

pub struct BinaryDiagnostic {
    data: Vec<Bin>,
}

impl crate::Advent for BinaryDiagnostic {
    fn new(data: &str) -> Self {
        BinaryDiagnostic {
            data: data
                .lines()
                .map(|d| d.bytes().map(|b| b - 48).collect::<Bin>())
                .collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 3 }

    fn part1(&self) -> usize {
        // create an array of integers with size = columns
        let mut counters = vec![0; 12];

        let number_lines = self.data.len();

        // iterate through the file
        for line in self.data.iter() {
            // add the value of each column to the array
            let mut i = 0;
            for &val in line.iter() {
                counters[i] += if val > 0 { 1 } else { 0 };
                i += 1;
            }
        }

        let mut gamma = 0;
        let mut epsilon = 0;

        for val in counters {
            gamma <<= 1;
            epsilon <<= 1;
            gamma |= if val < number_lines / 2 { 1 } else { 0 };
            epsilon |= if val > number_lines / 2 { 1 } else { 0 };
        }

        (gamma * epsilon) as usize
    }

    fn part2(&self) -> usize {
        let analyse = |criteria: bool| {
            let mut indexes = (0..self.data.len()).collect::<Vec<usize>>();
            let mut bit_index = 0;

            // repeat until there is only one remaining index
            while indexes.len() > 1 {
                let mut sum_in_col = 0;

                // count number of 'true' in column of remaining binary values
                indexes.iter().for_each(|&i| {
                    sum_in_col += if self.data[i][bit_index] > 0 { 1 } else { 0 };
                });

                let bit = if sum_in_col*2 >= indexes.len() { criteria } else { !criteria };

                indexes.retain(|&i| (self.data[i][bit_index] > 0) ^ bit);

                bit_index += 1;
            }

            self.data[indexes[0]].clone()
        };

        let bin_to_u32 = |bin: Bin| {
            let mut res = 0;

            for val in bin {
                res <<= 1;
                res |= if val > 0 { 1 } else { 0 }
            };

            res
        };

        let oxygen = bin_to_u32(analyse(true));
        let co2 = bin_to_u32(analyse(false));

        (oxygen * co2) as usize
    }
}
