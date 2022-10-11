use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn sub(&self, other: &Point) -> Point {
        Point {
            x: (self.x - other.x).abs() + 1,
            y: (self.y - other.y).abs() + 1,
            z: (self.z - other.z).abs() + 1,
        }
    }
}

#[derive(Copy, Clone)]
struct Command {
    state: bool,
    a: Point,
    b: Point,
}

impl Command {
    fn get_in_range(&self) -> Vec<Point> {
        let mut result = Vec::new();
        for x in self.a.x..=self.b.x {
            for y in self.a.y..=self.b.y {
                for z in self.a.z..=self.b.z {
                    result.push(Point { x, y, z });
                }
            }
        }
        result
    }
    fn is_in_small_range(&self) -> bool {
        self.a.x >= -50 && self.a.x <= 50 &&
            self.a.y >= -50 && self.a.y <= 50 &&
            self.a.z >= -50 && self.a.z <= 50 &&
            self.b.x >= -50 && self.b.x <= 50 &&
            self.b.y >= -50 && self.b.y <= 50 &&
            self.b.z >= -50 && self.b.z <= 50
    }

    fn is_overlapping(&self, other: &Command) -> bool {
        ((self.a.x >= other.a.x && self.a.x <= other.b.x) || (self.b.x >= other.a.x && self.b.x <= other.b.x) || (self.a.x <= other.a.x && self.b.x >= other.b.x)) &&
            ((self.a.y >= other.a.y && self.a.y <= other.b.y) || (self.b.y >= other.a.y && self.b.y <= other.b.y) || (self.a.y <= other.a.y && self.b.y >= other.b.y)) &&
            ((self.a.z >= other.a.z && self.a.z <= other.b.z) || (self.b.z >= other.a.z && self.b.z <= other.b.z) || (self.a.z <= other.a.z && self.b.z >= other.b.z))
    }

    fn turn_on(&self, other: &Command) -> Vec<Command> {
        let mut result = Vec::new();

        for c in Command::cut_up(other, self) {
            if !c.is_overlapping(other) {
                result.push(c);
            }
        }

        return result;
    }

    fn turn_off(&self, other: &Command) -> Vec<Command> {
        let mut result = Vec::new();

        for c in Command::cut_up(other, self) {
            if !c.is_overlapping(other) {
                result.push(c);
            }
        }

        return result;
    }

    fn get_size(&self) -> usize {
        let area = self.a.sub(&self.b);
        (area.x * area.y * area.z) as usize
    }

    fn cut_up(target: &Command, to_cut: &Command) -> Vec<Command> {
        // cut along x
        let mut along_x = Vec::new();
        if to_cut.b.x >= target.a.x && to_cut.b.x <= target.b.x && to_cut.a.x < target.a.x {
            let mut new_c = to_cut.clone();
            new_c.b.x = target.a.x - 1;
            along_x.push(new_c);
            let mut new_c = to_cut.clone();
            new_c.a.x = target.a.x;
            along_x.push(new_c);
        } else if to_cut.a.x >= target.a.x && to_cut.a.x <= target.b.x && to_cut.b.x > target.b.x {
            let mut new_c = to_cut.clone();
            new_c.b.x = target.b.x;
            along_x.push(new_c);
            let mut new_c = to_cut.clone();
            new_c.a.x = target.b.x + 1;
            along_x.push(new_c);
        } else if to_cut.a.x < target.a.x && to_cut.b.x > target.b.x {
            let mut new_c = to_cut.clone();
            new_c.b.x = target.a.x - 1;
            along_x.push(new_c);
            let mut new_c = to_cut.clone();
            new_c.a.x = target.a.x;
            new_c.b.x = target.b.x;
            along_x.push(new_c);
            let mut new_c = to_cut.clone();
            new_c.a.x = target.b.x + 1;
            along_x.push(new_c);
        } else { along_x.push(to_cut.clone()) }

        // cut along y
        let mut along_y = Vec::new();
        for &block in &along_x {
            if block.b.y >= target.a.y && block.b.y <= target.b.y && block.a.y < target.a.y {
                let mut new_c = block.clone();
                new_c.b.y = target.a.y - 1;
                along_y.push(new_c);
                let mut new_c = block.clone();
                new_c.a.y = target.a.y;
                along_y.push(new_c);
            } else if block.a.y >= target.a.y && block.a.y <= target.b.y && block.b.y > target.b.y {
                let mut new_c = block.clone();
                new_c.b.y = target.b.y;
                along_y.push(new_c);
                let mut new_c = block.clone();
                new_c.a.y = target.b.y + 1;
                along_y.push(new_c);
            } else if block.a.y < target.a.y && block.b.y > target.b.y {
                let mut new_c = block.clone();
                new_c.b.y = target.a.y - 1;
                along_y.push(new_c);
                let mut new_c = block.clone();
                new_c.a.y = target.a.y;
                new_c.b.y = target.b.y;
                along_y.push(new_c);
                let mut new_c = block.clone();
                new_c.a.y = target.b.y + 1;
                along_y.push(new_c);
            } else { along_y.push(block.clone()) }
        }

        // cut along z
        let mut result = Vec::new();
        for &block in &along_y {
            if block.b.z >= target.a.z && block.b.z <= target.b.z && block.a.z < target.a.z {
                let mut new_c = block.clone();
                new_c.b.z = target.a.z - 1;
                result.push(new_c);
                let mut new_c = block.clone();
                new_c.a.z = target.a.z;
                result.push(new_c);
            } else if block.a.z >= target.a.z && block.a.z <= target.b.z && block.b.z > target.b.z {
                let mut new_c = block.clone();
                new_c.b.z = target.b.z;
                result.push(new_c);
                let mut new_c = block.clone();
                new_c.a.z = target.b.z + 1;
                result.push(new_c);
            } else if block.a.z < target.a.z && block.b.z > target.b.z {
                let mut new_c = block.clone();
                new_c.b.z = target.a.z - 1;
                result.push(new_c);
                let mut new_c = block.clone();
                new_c.a.z = target.a.z;
                new_c.b.z = target.b.z;
                result.push(new_c);
                let mut new_c = block.clone();
                new_c.a.z = target.b.z + 1;
                result.push(new_c);
            } else { result.push(block.clone()) }
        }

        result
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
                    a: Point {
                        x: first.nth(1).unwrap().split("..").next().unwrap().parse().unwrap(),
                        y: first.next().unwrap().split("..").next().unwrap().parse().unwrap(),
                        z: first.next().unwrap().split("..").next().unwrap().parse().unwrap(),
                    },
                    b: Point {
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

        for c in &self.commands {
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
        let mut areas_on: Vec<Command> = Vec::new();

        for &c in &self.commands {
            if c.state {
                let mut areas_on_new = vec![c];
                let mut was_modified = true;
                'a: while was_modified {
                    was_modified = false;
                    for area in &areas_on {
                        for i in 0..areas_on_new.len() {
                            let area_new = &areas_on_new[i];
                            if area_new.is_overlapping(area) {
                                areas_on_new.append(&mut area_new.turn_on(area));
                                areas_on_new.remove(i);
                                was_modified = true;
                                continue 'a;
                            }
                        }
                    }
                }
                areas_on.append(&mut areas_on_new);
            } else {
                let mut was_modified = true;
                while was_modified {
                    was_modified = false;
                    for i in 0..areas_on.len() {
                        if areas_on[i].is_overlapping(&c) {
                            areas_on.append(&mut areas_on[i].turn_off(&c));
                            areas_on.remove(i);
                            was_modified = true;
                            break;
                        }
                    }
                }
            }
        }

        areas_on.iter().map(|area| area.get_size()).sum()
    }
}