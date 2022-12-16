use std::collections::HashSet;

struct Sensor {
    x: isize,
    y: isize,
    range: isize,
}

impl Sensor {
    fn next_increment(&self, x: isize, y: isize) -> isize {
        self.range - manhattan_dist((self.x, self.y), (x, y)) + 1
    }
}

pub struct BeaconExclusionZone {
    data: Vec<Sensor>,
    beacons_on_line: HashSet<isize>,
    x_min: isize,
    x_max: isize,
}

impl crate::Advent for BeaconExclusionZone {
    fn new(data: &str) -> Self {
        let mut beacons_on_line = HashSet::new();
        let mut x_min = isize::MAX;
        let mut x_max = isize::MIN;

        BeaconExclusionZone {
            data: data.lines().map(|l| {
                let indexes = all_indexes(l, '=');
                let x = get_value(l.as_bytes(), indexes[0]);
                let y = get_value(l.as_bytes(), indexes[1]);
                let u = get_value(l.as_bytes(), indexes[2]);
                let v = get_value(l.as_bytes(), indexes[3]);
                let range = manhattan_dist((x, y), (u, v));

                if x + range > x_max { x_max = x + range }
                if x - range < x_min { x_min = x - range }

                if v == 2000000 { beacons_on_line.insert(u); }

                Sensor { x, y, range }
            }).collect(),
            beacons_on_line,
            x_min,
            x_max,
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        15
    }

    fn part1(&self) -> usize {
        let mut covered = 0;
        let y = 2000000;

        let mut x = self.x_min;

        'a: while x <= self.x_max {
            for s in &self.data {
                let increment = s.next_increment(x, y);
                if increment > 0 {
                    for &b in &self.beacons_on_line {
                        if b >= x && b <= x + increment { covered -= 1 }
                    }

                    covered += increment;
                    x += increment;
                    continue 'a;
                }
            }
            x += 1
        }

        covered as usize
    }

    fn part2(&self) -> usize {
        let mut x_ = 0;
        let mut y_ = 0;
        // let max = 20;
        let max = 4000000;

        'a: for y in 0..=max {
            let mut x = 0;
            'b: while x <= max {
                for s in &self.data {
                    let next = s.next_increment(x, y);
                    if next > 0 {
                        x += next;
                        continue 'b;
                    }
                }
                x_ = x;
                y_ = y;

                break 'a;
            }
        }

        (x_ * 4000000 + y_) as usize
    }
}

fn manhattan_dist(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn get_value(s: &[u8], index: usize) -> isize {
    let mut is_neg = false;
    let mut result = 0;
    let mut i = index;

    loop {
        i += 1;
        if i >= s.len() || s[i] as char == ',' || s[i] as char == ':' {
            break;
        } else if s[i] as char == '-' {
            is_neg = true;
            continue;
        }

        result = result * 10 + (s[i] as char).to_digit(10).unwrap() as isize;
    }

    if is_neg { result * -1 } else { result }
}

fn all_indexes(s: &str, pattern: char) -> Vec<usize> {
    let mut results = vec![];
    let mut i = 0;

    for c in s.chars() {
        if c == pattern { results.push(i) }
        i += 1;
    }

    results
}