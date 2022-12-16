use priority_queue::PriorityQueue;

#[derive(Debug, Hash, Eq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

pub struct HillClimbing {
    data: Vec<Vec<isize>>,
    start: Point,
    end: Point,
}

impl crate::Advent for HillClimbing {
    fn new(data: &str) -> Self {
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };
        let mut y: isize = -1;

        let data = data.lines().map(|l| {
            y += 1;
            let mut x: isize = -1;
            l.chars().map(|c| {
                x += 1;
                match c {
                    'S' => {
                        start = Point { x: x as usize, y: y as usize };
                        0
                    }
                    'E' => {
                        end = Point { x: x as usize, y: y as usize };
                        25
                    }
                    _ => c as isize - ('a' as isize),
                }
            }).collect()
        }).collect();

        HillClimbing {
            data,
            start,
            end,
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        12
    }

    fn part1(&self) -> usize {
        find_shortest_path(&self.data, self.start, self.end)
    }

    fn part2(&self) -> usize {
        let mut shortest = find_shortest_path(&self.data, self.start, self.end);
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.data[y][x] > 0 { continue; }
                let length = find_shortest_path(&self.data, Point { x, y }, self.end);
                if length < shortest { shortest = length; }
            }
        }
        shortest
    }
}

fn find_shortest_path(data: &Vec<Vec<isize>>, start: Point, end: Point) -> usize {
    let mut queue = PriorityQueue::new();
    let mut weights = vec![vec![std::isize::MIN; data[0].len()]; data.len()];

    queue.push(start, 0);
    weights[start.y][start.x] = 0;

    'a: loop {
        match queue.pop() {
            Some(point) => {
                for p in get_adjacent_points(data, &weights, &point.0) {
                    weights[p.0.y][p.0.x] = p.1;
                    queue.push(p.0, p.1);
                    if p.0.x == end.x && p.0.y == end.y {
                        break 'a;
                    }
                }
            }
            None => {
                break 'a;
            }
        }
    }

    weights[end.y][end.x].abs() as usize
}

fn get_adjacent_points(field: &Vec<Vec<isize>>, weights: &Vec<Vec<isize>>, p: &Point) -> Vec<(Point, isize)> {
    let mut result = vec![];
    let max_x = field[0].len();
    let max_y = field.len();

    if p.x > 0 && weights[p.y][p.x] - 1 > weights[p.y][p.x - 1] && (field[p.y][p.x - 1] - field[p.y][p.x]) <= 1 {
        result.push((Point { x: p.x - 1, y: p.y }, weights[p.y][p.x] - 1))
    }

    if p.x + 1 < max_x && weights[p.y][p.x] - 1 > weights[p.y][p.x + 1] && (field[p.y][p.x + 1] - field[p.y][p.x]) <= 1 {
        result.push((Point { x: p.x + 1, y: p.y }, weights[p.y][p.x] - 1))
    }

    if p.y > 0 && weights[p.y][p.x] - 1 > weights[p.y - 1][p.x] && (field[p.y - 1][p.x] - field[p.y][p.x]) <= 1 {
        result.push((Point { x: p.x, y: p.y - 1 }, weights[p.y][p.x] - 1))
    }

    if p.y + 1 < max_y && weights[p.y][p.x] - 1 > weights[p.y + 1][p.x] && (field[p.y + 1][p.x] - field[p.y][p.x]) <= 1 {
        result.push((Point { x: p.x, y: p.y + 1 }, weights[p.y][p.x] - 1))
    }


    result
}