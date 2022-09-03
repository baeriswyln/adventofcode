use std::collections::HashMap;

struct Point {
    x: usize,
    y: usize,
}

struct Line {
    start: Point,
    end: Point,
}

pub struct HydrothermalAdventure {
    data: Vec<Line>,
}

impl crate::Advent for HydrothermalAdventure {
    fn new(data: &str) -> Self {
        HydrothermalAdventure {
            data: data.lines().map(|line| {
                let coordinates = line
                    .split(" -> ")
                    .flat_map(|p| p
                        .split(",")
                        .flat_map(|n| n.parse::<usize>())
                        .collect::<Vec<usize>>())
                    .collect::<Vec<usize>>();
                Line {
                    start: Point {
                        x: coordinates[0],
                        y: coordinates[1],
                    },
                    end: Point {
                        x: coordinates[2],
                        y: coordinates[3],
                    },
                }
            }).collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 5 }

    fn part1(&self) -> usize {
        let mut field = HashMap::new();

        for line in &self.data {
            let mut x = line.start.x;
            let mut y = line.start.y;
            if !(line.start.x == line.end.x || line.start.y == line.end.y) { continue; };

            while x != line.end.x || y != line.end.y {
                activate_point(&mut field, x, y);
                approach(&mut x, line.end.x);
                approach(&mut y, line.end.y);
            }
            activate_point(&mut field, x, y)
        }

        let mut count = 0;

        for (_, line) in field {
            for (_, entry) in line {
                if entry > 1 { count += 1 };
            }
        }

        count
    }

    fn part2(&self) -> usize {
        let mut field = HashMap::new();

        for line in &self.data {
            let mut x = line.start.x;
            let mut y = line.start.y;

            while x != line.end.x || y != line.end.y {
                activate_point(&mut field, x, y);
                approach(&mut x, line.end.x);
                approach(&mut y, line.end.y);
            }
            activate_point(&mut field, x, y)
        }

        let mut count = 0;

        for (_, line) in field {
            for (_, entry) in line {
                if entry > 1 { count += 1 };
            }
        }

        count
    }
}

fn activate_point(map: &mut HashMap<usize, HashMap<usize, usize>>, x: usize, y: usize) {
    map.entry(x)
        .or_insert(HashMap::new())
        .entry(y)
        .and_modify(|val| *val += 1)
        .or_insert(1);
}

fn approach(val : &mut usize, target : usize){
    if *val != target {
        if *val < target {
            *val += 1;
        } else {
            *val -= 1;
        }
    }
}