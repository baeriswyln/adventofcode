struct Fold {
    axis: char,
    index: usize,
}

struct Point {
    x: usize,
    y: usize,
}

pub struct Origami {
    dots: Vec<Point>,
    folds: Vec<Fold>,
}

impl crate::Advent for Origami {
    fn new(data: &str) -> Self {
        let mut split = data.split("\r\n\r\n");

        let dots = split.next().unwrap().lines().map(|line| {
            let mut line_split = line.split(",");
            Point {
                x: line_split.next().unwrap().parse().unwrap(),
                y: line_split.next().unwrap().parse().unwrap(),
            }
        }).collect();

        let folds = split.next().unwrap().lines().map(|line| {
            Fold {
                axis: line.chars().nth(11).unwrap(),
                index: line.split("=").nth(1).unwrap().parse().unwrap(),
            }
        }).collect();

        Origami {
            dots,
            folds,
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 13 }

    fn part1(&self) -> usize {
        // convert the points into a field of boolean where the value will be set to true if a point
        // exists at that cell
        let max_x = self.dots.iter().map(|p| p.x).max().unwrap();
        let max_y = self.dots.iter().map(|p| p.y).max().unwrap();
        let mut map = vec![vec![false; max_y + 1]; max_x + 1];

        for p in self.dots.iter() {
            map[p.x][p.y] |= true;
        }

        // make the fold
        let field = fold(&map, &self.folds[0]);

        // compute the number of dots left on the new field
        let mut result = 0;
        for x in 0..field.len() {
            for y in 0..field[x].len() {
                if field[x][y] { result += 1 }
            }
        }
        result
    }

    fn part2(&self) -> usize {
        // prepare the field
        let max_x = self.dots.iter().map(|p| p.x).max().unwrap();
        let max_y = self.dots.iter().map(|p| p.y).max().unwrap();
        let mut map = vec![vec![false; max_y + 1]; max_x + 1];

        for p in self.dots.iter() {
            map[p.x][p.y] |= true;
        }

        for f in &self.folds {
            map = fold(&map, f);
        }

        for y in 0..map[0].len() {
            for x in 0..map.len() {
                if x%5==0 {print!("  ")}
                print!("{}", if map[x][y] {"#"} else {" "});
            }
            println!();
        }


        0
    }
}


fn fold(field: &Vec<Vec<bool>>, fold: &Fold) -> Vec<Vec<bool>> {
    let mut max_x = field.len();
    let mut max_y = field[0].len();

    // adapt the field size to take in account the fold
    if fold.axis == 'x' {
        max_x = fold.index;
    } else {
        max_y = fold.index;
    }

    // create the results field
    let mut results = vec![vec![false; max_y]; max_x];

    // go through all cells on the input field and either mirror it or leave it where it is
    for x in 0..field.len() {
        for y in 0..field[x].len() {
            if !field[x][y] { continue; }
            if x > max_x {
                results[(2 * fold.index) - x][y] |= true;
            } else if y > max_y {
                results[x][(2 * fold.index) - y] |= true;
            } else {
                results[x][y] |= true;
            }
        }
    }

    results
}