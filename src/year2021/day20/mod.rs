pub struct TrenchMap {
    enhancement: Vec<bool>,
    picture: Vec<Vec<bool>>,
}

impl crate::Advent for TrenchMap {
    fn new(data: &str) -> Self {
        let mut split = data.split("\r\n\r\n");
        TrenchMap {
            enhancement: split.next().unwrap().chars().map(|c| c == '#').collect(),
            picture: split.next().unwrap().lines().map(|l|
                l.chars().map(|c| c == '#').collect()).collect(),
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 20 }

    fn part1(&self) -> usize {
        let enhance_steps = 2;
        let mut picture = self.picture.clone();

        for i in 0..enhance_steps {
            picture = enhance(&self.enhancement, &picture,i);
        }

        picture.iter().map(|l| l.iter().filter(|&p| *p).count()).sum()
    }

    fn part2(&self) -> usize {
        let enhance_steps = 50;
        let mut picture = self.picture.clone();

        for i in 0..enhance_steps {
            picture = enhance(&self.enhancement, &picture,i);
        }

        picture.iter().map(|l| l.iter().filter(|&p| *p).count()).sum()
    }
}

fn enhance(algo: &Vec<bool>, picture: &Vec<Vec<bool>>, index: usize) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; picture[0].len() + 2]; picture.len() + 2];

    for i in 0..result.len() {
        for j in 0..result[0].len() {
            result[i][j] = algo[get_val((i as isize - 2, j as isize - 2),
                                        (i as isize, j as isize), picture, index)]
        }
    }

    result
}

fn get_val(a: (isize, isize), b: (isize, isize), picture: &Vec<Vec<bool>>, index: usize) -> usize {
    let mut val = 0;
    for i in a.0..=b.0 {
        for j in a.1..=b.1 {
            val <<= 1;
            if i >= 0 && i < picture.len() as isize && j >= 0 && j < picture[0].len() as isize {
                if picture[i as usize][j as usize] { val |= 1 }
            } else if index % 2 == 1 { val |= 1 }
        }
    }

    val
}