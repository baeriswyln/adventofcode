use std::collections::HashMap;

pub struct RegolithReservoir {
    field: HashMap<(usize, usize), bool>,
    max_y: usize,
}

impl crate::Advent for RegolithReservoir {
    fn new(data: &str) -> Self {
        let mut field = HashMap::new();
        let mut max_y = 0;

        let points: Vec<Vec<(usize, usize)>> = data.lines().map(|l|
            l.split(" -> ").map(|p| {
                let mut iter = p.split(",");
                (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
            }).collect()
        ).collect();

        for l in points {
            for i in 0..(l.len() - 1) {
                let mut start_x = l[i].0;
                let mut start_y = l[i].1;
                let end_x = l[i + 1].0;
                let end_y = l[i + 1].1;

                if start_y > max_y { max_y = start_y }
                if end_y > max_y { max_y = end_y }

                field.insert(l[i], true);
                field.insert(l[i + 1], true);

                if start_x != end_x {
                    while start_x != end_x {
                        start_x += ((end_x as isize - start_x as isize) / start_x.abs_diff(end_x)
                            as isize) as usize;
                        field.insert((start_x, start_y), true);
                    }
                } else {
                    while start_y != end_y {
                        start_y += ((end_y as isize - start_y as isize) / start_y.abs_diff(end_y)
                            as isize) as usize;
                        field.insert((start_x, start_y), true);
                    }
                }
            }
        }

        RegolithReservoir {
            field,
            max_y,
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        14
    }

    fn part1(&self) -> usize {
        let mut counter = 0;
        let mut field = self.field.clone();

        'a: loop {
            let mut x = 500;
            let mut y = 0;
            loop {
                if field.contains_key(&(x, y + 1)) {
                    if !field.contains_key(&(x - 1, y + 1)) {
                        x -= 1;
                        y += 1;
                        continue;
                    } else if !field.contains_key(&(x + 1, y + 1)) {
                        x += 1;
                        y += 1;
                        continue;
                    } else {
                        field.insert((x, y), true);
                        break;
                    }
                } else if y > self.max_y {
                    break 'a;
                } else {
                    y += 1;
                }
            }

            counter += 1;
        }

        counter
    }

    fn part2(&self) -> usize {
        let mut counter = 0;
        let mut field = self.field.clone();

        'a: loop {
            let mut x = 500;
            let mut y = 0;
            loop {
                if field.contains_key(&(x, y + 1)) {
                    if !field.contains_key(&(x - 1, y + 1)) {
                        x -= 1;
                        y += 1;
                        continue;
                    } else if !field.contains_key(&(x + 1, y + 1)) {
                        x += 1;
                        y += 1;
                        continue;
                    } else {
                        field.insert((x, y), true);
                        if y == 0{
                            counter += 1;
                            break 'a;
                        }
                        break;
                    }
                } else if y >= self.max_y + 2 {
                    field.insert((x, y - 1), true);
                    break;
                } else {
                    y += 1;
                }
            }

            counter += 1;
        }

        counter
    }
}