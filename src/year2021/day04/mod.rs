type Board = Vec<Vec<u8>>;

struct Bingo {
    random_numbers: Vec<u8>,
    boards: Vec<Board>,
}

pub struct GiantSquid {
    data: Bingo,
}

impl crate::Advent for GiantSquid {
    fn new(data: &str) -> Self {
        let mut iter = data.split("\r\n\r\n\r\n");
        GiantSquid {
            data: Bingo {
                random_numbers: iter.next().unwrap()
                    .split(',')
                    .flat_map(|n| n.parse())
                    .collect::<Vec<u8>>(),
                boards: iter.next().unwrap()
                    .split("\r\n\r\n")
                    .map(
                        |grid| grid.lines()
                            .map(|line| line.split_whitespace()
                                .flat_map(|n| n.parse::<u8>())
                                .collect::<Vec<u8>>())
                            .collect::<Vec<Vec<u8>>>())
                    .collect(),
            }
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 4 }

    fn part1(&self) -> usize {
        let mut played_numbers = Vec::new();

        for x in self.data.random_numbers.iter() {
            played_numbers.push(*x);

            for board in self.data.boards.iter() {
                if check_board(&played_numbers, board) {
                    return get_sum_of_unmarked(&played_numbers, board) * <u8 as
                    Into<usize>>::into(*x);
                }
            }
        }

        0
    }

    fn part2(&self) -> usize {
        let mut remaining_boards = (0..self.data.boards.len()).collect::<Vec<usize>>();
        let mut played_numbers = Vec::new();
        let mut i = 0;

        // sort out all but one board. the remaining board will win last
        while remaining_boards.len() > 1 {
            played_numbers.push(self.data.random_numbers[i]);
            remaining_boards.retain(|&i| !check_board(&played_numbers, &self.data.boards[i]));

            i += 1;
        }

        // continue to draw numbers until this board has won
        let last_board = &self.data.boards[played_numbers[0] as usize];
        while !check_board(&played_numbers, last_board) {
            played_numbers.push(self.data.random_numbers[i]);
            i += 1;
        }

        get_sum_of_unmarked(&played_numbers, last_board) * (played_numbers[played_numbers.len() -
            1] as usize)
    }
}

fn check_board(numbers: &Vec<u8>, board: &Board) -> bool {
    'outer: for i in 0..5 {
        for j in 0..5 {
            if !numbers.contains(&board[i][j]) {
                continue 'outer;
            };
        }
        return true;
    }

    'outer: for i in 0..5 {
        for j in 0..5 {
            if !numbers.contains(&board[j][i]) { continue 'outer; };
        }
        return true;
    }

    for i in 0..5 {
        if !numbers.contains(&board[i][i]) { break; };
        if i == 4 {
            return true;
        }
    }
    for i in 0..5 {
        if !numbers.contains(&board[i][4 - i]) { break; };
        if i == 4 {
            return true;
        }
    }
    false
}

fn get_sum_of_unmarked(numbers: &Vec<u8>, board: &Board) -> usize {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !numbers.contains(&board[i][j]) { sum += board[i][j] as usize };
        }
    }
    sum
}