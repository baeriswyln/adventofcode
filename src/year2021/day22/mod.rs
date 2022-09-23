use std::collections::HashMap;

struct Point {
    x: isize,
    y: isize,
    z: isize,
}

struct Command {
    state: bool,
    point_a: Point,
    point_b: Point,
}

impl Command {
    fn get_in_range(&self) -> Vec<Point> {
        let mut result = Vec::new();
        for x in self.point_a.x..=self.point_b.x {
            for y in self.point_a.y..=self.point_b.y {
                for z in self.point_a.z..=self.point_b.z {
                    result.push(Point { x, y, z });
                }
            }
        }
        result
    }
    fn is_in_small_range(&self) -> bool {
        self.point_a.x >= -50 && self.point_a.x <= 50 &&
        self.point_a.y >= -50 && self.point_a.y <= 50 &&
        self.point_a.z >= -50 && self.point_a.z <= 50 &&
        self.point_b.x >= -50 && self.point_b.x <= 50 &&
        self.point_b.y >= -50 && self.point_b.y <= 50 &&
        self.point_b.z >= -50 && self.point_b.z <= 50
    }
}

pub struct ReactorReboot {
    commands: Vec<Command>,
}

impl crate::Advent for ReactorReboot {
    fn new(data: &str) -> Self {
        ReactorReboot {
            commands: data.lines().map(|l| {
                let mut first = l.split("=");
                let mut last = l.split(",");
                Command {
                    state: l.chars().nth(1).unwrap() == 'n',
                    point_a: Point {
                        x: first.nth(1).unwrap().split("..").next().unwrap().parse().unwrap(),
                        y: first.next().unwrap().split("..").next().unwrap().parse().unwrap(),
                        z: first.next().unwrap().split("..").next().unwrap().parse().unwrap(),
                    },
                    point_b: Point {
                        x: last.next().unwrap().split("..").last().unwrap().parse().unwrap(),
                        y: last.next().unwrap().split("..").last().unwrap().parse().unwrap(),
                        z: last.next().unwrap().split("..").last().unwrap().parse().unwrap(),
                    },
                }
            }).collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 22 }

    fn part1(&self) -> usize {
        let mut cubes = HashMap::new();

        for c in &self.commands{
            if !c.is_in_small_range() { continue; }
            if c.state {
                for p in c.get_in_range() {
                    cubes.insert((p.x, p.y, p.z), 1);
                }
            } else {
                for p in c.get_in_range() {
                    cubes.remove(&(p.x, p.y, p.z));
                }
            }
        }

        cubes.len()
    }

    fn part2(&self) -> usize {
        0
    }
}