struct Board {
    markings: [[bool;5];5],
    numbers: Vec<Vec<u32>>,
}

impl Board {
    fn play(&self, number: u32) -> bool {
        let mut i = 0;
        for line in self.numbers {
            let mut j = 0;
            for cell in line {
                if cell == number {
                    markings[i][j] = true;
                    return true
                }
                j += 1;
            }
            i += 1;
        }
        false
    }

    fn won() -> bool{

    }

    fn get_sum_unmarked(&self) -> u32 {
        let mut result = 0;
        let mut i = 0;
        for line in self.numbers {
            let mut j = 0;
            for cell in line {
                if markings[i][j] {
                    result += cell;
                }
                j += 1;
            }
            i += 1;
        }

        result
    }
}

pub struct GiantSquid {
    random_numbers: Vec<u32>,
    boards: Vec<Board>
}

impl crate::Advent for GiantSquid {
    fn new(data: &str) -> Self  {
        let random_numbers = data
            .lines().next().expect("")
            .split(',').map(|n| n.parse::<u32>().expect("")).collect();
        let lines: Vec<&str> = data.lines().collect();

        let mut counter = 0;
        let mut numbers= Vec::new();
        let mut boards = Vec::new();

        for i in 2..lines.len() {
            if lines[i].len() < 5 {
                counter = 0;
                let n = numbers.clone();
                boards.push(Board { markings: [[false;5];5], numbers: n });
                numbers= Vec::new();
                continue;
            }
            numbers.push(lines[i]
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect(""))
                .collect());
            counter += 1;
        }

        GiantSquid{
            random_numbers,
            boards
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 4 }

    fn part1(&self) -> usize {
        let mut board: Board;
        let mut number = 0;

        'outer: for n in self.random_numbers {
            for b in self.boards {
                if b.play(n) && b.won() {
                    number = n;
                    board = b;
                    break 'outer;
                }
            }
        }

        (number * b.get_sum_unmarked)
    }

    fn part2(&self) -> usize {
        0
    }
}