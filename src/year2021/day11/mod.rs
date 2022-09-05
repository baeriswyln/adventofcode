struct Octopus {
    energy_level: u8,
    flashed: bool,
}

impl Octopus {
    fn reset(&mut self) { self.flashed = false; }
}

pub struct Dumbo {
    data: Vec<Vec<u8>>,
}

impl crate::Advent for Dumbo {
    fn new(data: &str) -> Self {
        Dumbo {
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

    fn day() -> u8 { 11 }

    fn part1(&self) -> usize {
        let mut field = Vec::new();

        for i in 0..self.data.len() {
            let mut line = Vec::new();
            for j in 0..self.data[i].len() {
                line.push(Octopus {
                    energy_level: self.data[i][j],
                    flashed: false,
                })
            }
            field.push(line)
        }

        let mut result = 0;
        for _ in 0..100 {
            for i in 0..field.len() {
                for j in 0..field.len() {
                    result += activate(&mut field, i, j)
                }
            }

            field.iter_mut().for_each(|l|
                l.iter_mut().for_each(|o| o.reset()))
        }
        result
    }

    fn part2(&self) -> usize {
        let mut field = Vec::new();

        for i in 0..self.data.len() {
            let mut line = Vec::new();
            for j in 0..self.data[i].len() {
                line.push(Octopus {
                    energy_level: self.data[i][j],
                    flashed: false,
                })
            }
            field.push(line)
        }

        let mut turn = 0;
        loop {
            turn += 1;
            for i in 0..field.len() {
                for j in 0..field.len() {
                    activate(&mut field, i, j);
                }
            }

            if field.iter().all(|l| l.iter().all(|o| o.flashed)) {
                break;
            }

            field.iter_mut().for_each(|l|
                l.iter_mut().for_each(|o| o.reset()))
        }
        turn
    }
}

fn activate(field: &mut Vec<Vec<Octopus>>, x: usize, y: usize) -> usize {
    let mut result = 0;
    if field[x][y].energy_level == 9 {
        result += 1;
        field[x][y].flashed = true;
        field[x][y].energy_level = 0;
        get_neighbors(field, x, y)
            .iter()
            .for_each(|p| result += activate(field, p.0, p.1))
    } else if !field[x][y].flashed {
        field[x][y].energy_level += 1;
    }
    result
}

fn get_neighbors(field: &Vec<Vec<Octopus>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if x > 0 {
        neighbors.push((x - 1, y));
        if y > 0 { neighbors.push((x - 1, y - 1)) }
        if y < field[x].len() - 1 { neighbors.push((x - 1, y + 1)) }
    };
    if y > 0 { neighbors.push((x, y - 1)) };
    if x < field.len() - 1 {
        neighbors.push((x + 1, y));
        if y > 0 { neighbors.push((x + 1, y - 1)) }
        if y < field[x].len() - 1 { neighbors.push((x + 1, y + 1)) }
    };
    if y < field[x].len() - 1 { neighbors.push((x, y + 1)) };

    neighbors
}