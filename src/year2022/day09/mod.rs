use std::collections::HashMap;

#[derive(Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn as_key(&self) -> (isize, isize) {
        (self.x, self.y)
    }
    fn approach(&mut self, other: &Point) {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        if dx >= 2 {
            self.x -= 1;
            if dy.abs() != 0 { self.y -= dy / dy.abs() }
        } else if dx <= -2 {
            self.x += 1;
            if dy.abs() != 0 { self.y -= dy / dy.abs() }
        } else if dy >= 2 {
            self.y -= 1;
            if dx.abs() != 0 { self.x -= dx / dx.abs() }
        } else if dy <= -2 {
            self.y += 1;
            if dx.abs() != 0 { self.x -= dx / dx.abs() }
        }
    }
}

struct Move {
    direction: char,
    steps: usize,
}

pub struct RopeBridge {
    moves: Vec<Move>,
}

impl crate::Advent for RopeBridge {
    fn new(data: &str) -> Self {
        RopeBridge {
            moves: data.lines().map(|l| {
                let mut iter = l.split(" ");
                Move {
                    direction: iter.next().unwrap().as_bytes()[0] as char,
                    steps: iter.next().unwrap().parse().unwrap(),
                }
            }
            ).collect()
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        9
    }

    fn part1(&self) -> usize {
        let mut visited = HashMap::new();
        let mut head = Point { x: 0, y: 0 };
        let mut tail = Point { x: 0, y: 0 };

        for m in &self.moves {
            for _ in 0..m.steps {
                match m.direction {
                    'U' => head.y += 1,
                    'D' => head.y -= 1,
                    'L' => head.x -= 1,
                    'R' => head.x += 1,
                    _ => ()
                }
                tail.approach(&head);
                visited.insert(tail.as_key(), true);
            }
        }

        visited.len()
    }

    fn part2(&self) -> usize {
        let mut visited = HashMap::new();

        let mut knots = vec![Point { x: 0, y: 0 }; 10];

        for m in &self.moves {
            for _ in 0..m.steps {
                match m.direction {
                    'U' => knots[0].y += 1,
                    'D' => knots[0].y -= 1,
                    'L' => knots[0].x -= 1,
                    'R' => knots[0].x += 1,
                    _ => ()
                }
                for i in 1..knots.len() {
                    let knot_prev = knots[i - 1].clone();
                    knots[i].approach(&knot_prev);
                }
                visited.insert(knots[knots.len() - 1].as_key(), true);
            }
        }

        visited.len()
    }
}
