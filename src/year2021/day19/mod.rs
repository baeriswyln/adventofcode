#[derive(Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

struct Scanner {
    points: Vec<Point>,
}

pub struct BeaconScanner {
    scanners: Vec<Scanner>,
}

impl crate::Advent for BeaconScanner {
    fn new(data: &str) -> Self {
        let scanners = data.split("").map(|part| {
            let mut lines = part.lines();
            let mut points = Vec::new();
            lines.next();
            for l in lines {
                let mut numbers = l.split(",");
                points.push(Point {
                    x: numbers.nth(0).unwrap().parse().unwrap(),
                    y: numbers.nth(1).unwrap().parse().unwrap(),
                    z: numbers.nth(2).unwrap().parse().unwrap(),
                })
            }
            Scanner {
                points
            }
        }).collect();

        BeaconScanner {
            scanners
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 19 }

    fn part1(&self) -> usize {
        let mut confirmed_absolute_points = Vec::new();
        let mut scanners = self.scanners.clone();

        // scanner 0 is the center. all others will be put around this.
        confirmed_absolute_points.push(scanners[0].points.clone());
        scanners.remove(0);

        // go through all remaining sensor data
        while scanners.len() > 0 {
            // get for each possible sensor if they are overlapping
            // if they are overlapping, add the absolute coordinates and store them
        }

        get_unique_points(&confirmed_absolute_points).len()
    }

    fn part2(&self) -> usize {
        0
    }
}

/// get all 24 possible transformations
fn transform() -> Vec<Vec<Point>> {
    Vec::new()
}

fn get_absolute_coordinates() -> Vec<Point> {
    Vec::new()
}

fn is_overlapping() -> bool {
    false
}

fn get_unique_points(collections: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut results = Vec::new();

    for &points in collections {
        'outer: for p in points {
            for r in results {
                if r.x == p.x && r.y == p.y && r.z == p.z { continue 'outer; }
            }
            results.push(p.clone());
        }
    }

    results
}