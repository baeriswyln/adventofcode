pub struct SmokeBasin {
    data: Vec<Vec<u8>>,
}

impl crate::Advent for SmokeBasin {
    fn new(data: &str) -> Self {
        SmokeBasin {
            data: data
                .lines()
                .map(|line|
                    line.chars()
                        .map(|c| (c as u8) - 48)
                        .collect())
                .collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 9 }

    fn part1(&self) -> usize {
        let mut sum_lows = 0;

        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                // check each point and see if all adjacent values are bigger than the current point
                if (i <= 0 || &self.data[i - 1][j] > &self.data[i][j]) &&
                    (j <= 0 || &self.data[i][j - 1] > &self.data[i][j]) &&
                    (i >= self.data.len() - 1 || &self.data[i + 1][j] > &self.data[i][j]) &&
                    (j >= self.data[i].len() - 1 || &self.data[i][j + 1] > &self.data[i][j]) {
                    sum_lows += 1 + self.data[i][j] as usize;
                }
            }
        }

        sum_lows
    }

    fn part2(&self) -> usize {
        let mut pool_sizes = Vec::new();
        // prepare a vector holding a flag that can mark cells as checked
        let mut status = vec![vec![false; self.data[0].len()]; self.data.len()];

        // compute for each cell the size of the pit it is in. each cell must only be part of one
        // pit
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                let pool_size = compute_pool_size(i, j, &self.data, &mut status);
                if pool_size > 0 { pool_sizes.push(pool_size) }
            }
        }

        // sort the array in ascending order and multiply the 3 largest values
        let len = pool_sizes.len();
        pool_sizes.sort();
        pool_sizes[len-1] * pool_sizes[len-2] * pool_sizes[len-3]
    }
}

fn compute_pool_size(x: usize, y: usize, data: &Vec<Vec<u8>>, status : &mut Vec<Vec<bool>>) ->
                                                                                           usize {
    // exit criteria are: value is 9, cell has already been checked
    if data[x][y] == 9 || status[x][y] { return 0; }
    // mark the cell as checked
    status[x][y] = true;

    // set the result to 1 as the cell that is currently evaluated is in a pit
    let mut result = 1;

    // recursively check all adjacent cells and add their pit size to the result
    if x > 0 { result += compute_pool_size(x - 1, y, &data, status) };
    if y > 0 { result += compute_pool_size(x, y - 1, &data, status) };
    if x < data.len() - 1 { result += compute_pool_size(x + 1, y, &data, status) };
    if y < data[x].len() - 1 { result += compute_pool_size(x, y + 1, &data, status) };

    result
}