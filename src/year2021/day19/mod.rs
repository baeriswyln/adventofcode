#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn diff(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn dist_manhattan(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

#[derive(Clone)]
struct Scanner {
    points: Vec<Point>,
}

pub struct BeaconScanner {
    scanners: Vec<Scanner>,
}

impl crate::Advent for BeaconScanner {
    fn new(data: &str) -> Self {
        let scanners = data.split("\r\n\r\n").map(|part| {
            let mut lines = part.lines();
            let mut points = Vec::new();
            lines.next();
            for l in lines {
                let mut numbers = l.split(",");
                points.push(Point {
                    x: numbers.next().unwrap().parse().unwrap(),
                    y: numbers.next().unwrap().parse().unwrap(),
                    z: numbers.next().unwrap().parse().unwrap(),
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
        get_unique_points(&get_all_beacons(&self.scanners).0).len()
    }

    fn part2(&self) -> usize {
        let scanners = get_all_beacons(&self.scanners).1;
        let mut max = 0;

        for i in 0..scanners.len() {
            for j in 0..scanners.len() {
                let dist = scanners[i].dist_manhattan(&scanners[j]);
                if dist > max { max = dist }
            }
        }

        max
    }
}

fn get_all_beacons(scanners: &Vec<Scanner>) -> (Vec<Vec<Point>>, Vec<Point>) {
    let mut confirmed_absolute_points = Vec::new();
    let mut scanner_pos = Vec::new();
    let mut scanners = scanners.clone();

    // scanner 0 is the center. all others will be put around this.
    confirmed_absolute_points.push(scanners[0].points.clone());
    scanners.remove(0);

    // go through all remaining sensor data
    while scanners.len() > 0 {
        'a: for i in 0..scanners.len() {
            for j in 0..confirmed_absolute_points.len() {
                // get for each possible sensor if they are overlapping
                match is_overlapping(&confirmed_absolute_points[j], &scanners[i].points) {
                    Some(points) => {
                        // if they are overlapping, get the absolute coordinates and store them
                        confirmed_absolute_points.push(points.0);
                        scanner_pos.push(points.1);
                        scanners.remove(i);
                        break 'a;
                    }
                    None => ()
                }
            }
        }
    }

    (confirmed_absolute_points, scanner_pos)
}

/// manually get all 24 possible transformations (don't even ask...)
fn transform(points: &Vec<Point>) -> Vec<Vec<Point>> {
    let mut results = Vec::new();
    results.push(points.iter().map(|p| Point { x: p.x, y: p.y, z: p.z }).collect());
    results.push(points.iter().map(|p| Point { x: p.x, y: p.z, z: -p.y }).collect());
    results.push(points.iter().map(|p| Point { x: p.x, y: -p.y, z: -p.z }).collect());
    results.push(points.iter().map(|p| Point { x: p.x, y: -p.z, z: p.y }).collect());
    results.push(points.iter().map(|p| Point { x: -p.x, y: p.y, z: -p.z }).collect());
    results.push(points.iter().map(|p| Point { x: -p.x, y: -p.z, z: -p.y }).collect());
    results.push(points.iter().map(|p| Point { x: -p.x, y: -p.y, z: p.z }).collect());
    results.push(points.iter().map(|p| Point { x: -p.x, y: p.z, z: p.y }).collect());
    results.push(points.iter().map(|p| Point { x: p.y, y: -p.x, z: p.z }).collect());
    results.push(points.iter().map(|p| Point { x: p.y, y: p.z, z: p.x }).collect());
    results.push(points.iter().map(|p| Point { x: p.y, y: p.x, z: -p.z }).collect());
    results.push(points.iter().map(|p| Point { x: p.y, y: -p.z, z: -p.x }).collect());
    results.push(points.iter().map(|p| Point { x: -p.y, y: p.x, z: p.z }).collect());
    results.push(points.iter().map(|p| Point { x: -p.y, y: p.z, z: -p.x }).collect());
    results.push(points.iter().map(|p| Point { x: -p.y, y: -p.x, z: -p.z }).collect());
    results.push(points.iter().map(|p| Point { x: -p.y, y: -p.z, z: p.x }).collect());
    results.push(points.iter().map(|p| Point { x: p.z, y: p.x, z: p.y }).collect());
    results.push(points.iter().map(|p| Point { x: p.z, y: p.y, z: -p.x }).collect());
    results.push(points.iter().map(|p| Point { x: p.z, y: -p.x, z: -p.y }).collect());
    results.push(points.iter().map(|p| Point { x: p.z, y: -p.y, z: p.x }).collect());
    results.push(points.iter().map(|p| Point { x: -p.z, y: p.x, z: -p.y }).collect());
    results.push(points.iter().map(|p| Point { x: -p.z, y: -p.y, z: -p.x }).collect());
    results.push(points.iter().map(|p| Point { x: -p.z, y: -p.x, z: p.y }).collect());
    results.push(points.iter().map(|p| Point { x: -p.z, y: p.y, z: p.x }).collect());
    results
}

/// move all input points such that point (rel) that is part of the points vector, will have the
/// coordinates of (abs). moving in 3d space
fn get_absolute_coordinates(points: &Vec<Point>, rel: &Point, abs: &Point) -> Vec<Point> {
    let d_x = abs.x - rel.x;
    let d_y = abs.y - rel.y;
    let d_z = abs.z - rel.z;

    points.iter().map(|p| Point {
        x: p.x + d_x,
        y: p.y + d_y,
        z: p.z + d_z,
    }).collect()
}

/// check if the two vectors have at least 12 identical points
fn is_overlapping(confirmed: &Vec<Point>, to_confirm: &Vec<Point>) -> Option<(Vec<Point>, Point)> {
    let transformed = transform(to_confirm);

    for t in &transformed {
        for c in confirmed {
            for p in t {
                let absolute = get_absolute_coordinates(t, p, c);
                if count_identical(confirmed, &absolute) >= 12 {
                    return Some((absolute, c.diff(p)));
                }
            }
        }
    }

    None
}

/// count the number of points that are both in a_vec and b_vec, based on their coordinates
fn count_identical(a_vec: &Vec<Point>, b_vec: &Vec<Point>) -> usize {
    a_vec.iter().map(|a| {
        for b in b_vec { if a.x == b.x && a.y == b.y && a.z == b.z { return 1; } }
        0
    }).sum()
}

/// get the number of unique points (unique coordinates)
fn get_unique_points(collections: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut results: Vec<Point> = Vec::new();

    for points in collections {
        'outer: for p in points {
            for r in &results {
                if r.x == p.x && r.y == p.y && r.z == p.z { continue 'outer; }
            }
            results.push(p.clone());
        }
    }

    results
}