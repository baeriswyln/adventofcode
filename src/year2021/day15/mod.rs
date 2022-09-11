use priority_queue::PriorityQueue;

pub struct Chiton {
    field: Vec<Vec<usize>>,
}

impl crate::Advent for Chiton {
    fn new(data: &str) -> Self {
        Chiton {
            field: data
                .lines()
                .map(|line| line
                    .chars()
                    .map(|c| ((c as usize) - 48))
                    .collect())
                .collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 15 }

    fn part1(&self) -> usize {
        compute_cost(&self.field)
    }

    fn part2(&self) -> usize {
        compute_cost(&get_large_map(&self.field, 5))
    }
}

fn compute_cost(field: &Vec<Vec<usize>>) -> usize {
    let len = field.len();
    let mut costs = vec![vec![usize::MAX; len]; len];
    let mut stack: PriorityQueue<(usize, usize), usize> = PriorityQueue::new();

    stack.push((1, 0), 10000 - field[1][0]);
    costs[1][0] = field[1][0];
    stack.push((0, 1), 10000 - field[0][1]);
    costs[0][1] = field[0][1];

    let mut counter = 0;

    while !stack.is_empty() {
        counter += 1;
        let (x, y) = stack.pop().unwrap().0;
        if x == len - 1 && y == len - 1 { continue; }

        if x > 0 && costs[x - 1][y] > costs[x][y] + field[x - 1][y] {
            stack.push((x - 1, y), 10000 - costs[x][y] + field[x - 1][y]);
            costs[x - 1][y] = costs[x][y] + field[x - 1][y];
        }
        if x < len - 1 && costs[x + 1][y] > costs[x][y] + field[x + 1][y] {
            stack.push((x + 1, y), 10000 - costs[x][y] + field[x + 1][y]);
            costs[x + 1][y] = costs[x][y] + field[x + 1][y];
        }
        if y > 0 && costs[x][y - 1] > costs[x][y] + field[x][y - 1] {
            stack.push((x, y - 1), 10000 - costs[x][y] + field[x][y - 1]);
            costs[x][y - 1] = costs[x][y] + field[x][y - 1];
        }
        if y < len - 1 && costs[x][y + 1] > costs[x][y] + field[x][y + 1] {
            stack.push((x, y + 1), 10000 - costs[x][y] + field[x][y + 1]);
            costs[x][y + 1] = costs[x][y] + field[x][y + 1];
        }
    }

    println!("{}", counter);
    costs[len - 1][len - 1] as usize
}

fn get_large_map(map: &Vec<Vec<usize>>, multiplier: usize) -> Vec<Vec<usize>> {
    let len_single = map.len();
    let len = len_single * multiplier;
    let mut result = vec![vec![0; len]; len];

    for x in 0..len {
        for y in 0..len {
            let val = map[x % len_single][y % len_single] - 1;
            let delta = (x / len_single) + (y / len_single);
            result[x][y] = ((val + delta) % 9) + 1;
        }
    }

    result
}