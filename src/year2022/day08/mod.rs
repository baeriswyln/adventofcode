pub struct TreeHouse {
    grid: Vec<Vec<isize>>,
}

impl crate::Advent for TreeHouse {
    fn new(data: &str) -> Self {
        TreeHouse {
            grid: data.lines().map(|l| l.chars().map(|c|
                c.to_string().parse().unwrap()).collect()
            ).collect()
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        8
    }

    fn part1(&self) -> usize {
        let dy = self.grid.len();
        let dx = self.grid[0].len();

        let mut field = vec![vec![false; dx]; dy];

        // in x direction
        for y in 0..dy {
            let mut left = -1;
            let mut right = -1;
            for x in 0..dx {
                if self.grid[y][x] > left {
                    left = self.grid[y][x];
                    field[y][x] = true
                }
                if self.grid[y][dx - 1 - x] > right {
                    right = self.grid[y][dx - 1 - x];
                    field[y][dx - 1 - x] = true
                }
            }
        }

        for x in 0..dx {
            let mut top = -1;
            let mut bottom = -1;
            for y in 0..dy {
                if self.grid[y][x] > top {
                    top = self.grid[y][x];
                    field[y][x] = true
                }
                if self.grid[dy - 1 - y][x] > bottom {
                    bottom = self.grid[dy - 1 - y][x];
                    field[dy - 1 - y][x] = true
                }
            }
        }

        field.iter().map(|l| l.iter().filter(|&&b| b).count()).sum()
    }

    fn part2(&self) -> usize {
        let dy = self.grid.len();
        let dx = self.grid[0].len();

        let mut result = 0;

        for y in 1..(dy - 1) {
            for x in 1..(dx - 1) {
                let h = self.grid[y][x];
                let mut score = 1;
                let mut counter = 0;

                for i in (0..x).rev() {
                    if self.grid[y][i] >= h {
                        counter += 1;
                        break;
                    }
                    counter += 1;
                }
                if counter > 0 { score *= counter }
                counter = 0;

                for i in (x + 1)..dx {
                    if self.grid[y][i] >= h {
                        counter += 1;
                        break;
                    }
                    counter += 1;
                }
                if counter > 0 { score *= counter }
                counter = 0;

                for i in (0..y).rev() {
                    if self.grid[i][x] >= h {
                        counter += 1;
                        break;
                    }
                    counter += 1;
                }
                if counter > 0 { score *= counter }
                counter = 0;

                for i in (y + 1)..dy {
                    if self.grid[i][x] >= h {
                        counter += 1;
                        break;
                    }
                    counter += 1;
                }
                if counter > 0 { score *= counter }

                if score > result { result = score }
            }
        }

        result
    }
}
