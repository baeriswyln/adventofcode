pub struct Snailfish {
    lines: Vec<Vec<i8>>,
}

impl crate::Advent for Snailfish {
    fn new(data: &str) -> Self {
        Snailfish {
            lines: data.lines().map(|line| {
                let mut chars = Vec::new();
                line.chars().for_each(|c| {
                    match c {
                        '[' => chars.push(-1),
                        ']' => chars.push(-2),
                        ',' => (),
                        _ => chars.push((c as i8) - 48)
                    };
                });
                chars
            }).collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 18 }

    fn part1(&self) -> usize {
        let mut number = self.lines[0].clone();

        for i in 1..self.lines.len() {
            number = reduce(add(&number, &self.lines[i]));
        }

        get_magnitude(&number)
    }

    fn part2(&self) -> usize {
        let mut max = 0;

        for i in 0..self.lines.len() {
            for j in 0..self.lines.len() {
                if i == j { continue; }
                let sum = get_magnitude(&reduce(add(&self.lines[i], &self.lines[j])));
                if sum > max { max = sum }
            }
        }

        max
    }
}

fn add(val1: &Vec<i8>, val2: &Vec<i8>) -> Vec<i8> {
    let mut result = Vec::new();
    result.push(-1);
    result.append(&mut val1.clone());
    result.append(&mut val2.clone());
    result.push(-2);
    result
}

fn reduce(number: Vec<i8>) -> Vec<i8> {
    let mut was_modified = true;
    let mut number = number.clone();

    'main: while was_modified {
        let mut brackets = 0;
        was_modified = false;

        for i in 0..number.len() {
            match number[i] {
                -1 => brackets += 1,
                -2 => brackets -= 1,
                _ => {
                    if brackets >= 5 {
                        number = explode(number, i);
                        was_modified = true;
                        continue 'main;
                    }
                }
            }
        }

        for i in 0..number.len() {
            if number[i] >= 10 {
                number = split(number, i);
                was_modified = true;
                break;
            }
        }
    }

    number
}

fn explode(number: Vec<i8>, index: usize) -> Vec<i8> {
    // store the values of the number that explodes
    let left = number[index];
    let right = number[index + 1];

    // remove the exploding value and construct new vector. exploding number is replaced with 0
    let mut result = number[..(index - 1)].to_vec();
    result.push(0);
    result.append(&mut number[(index + 3)..].to_vec());

    // add the left value to the next value to the left
    for i in (0..(index - 1)).rev() {
        if result[i] >= 0 {
            result[i] += left;
            break;
        }
    }

    // add the right value to the next value to the right
    for i in index..result.len() {
        if result[i] >= 0 {
            result[i] += right;
            break;
        }
    }

    result
}

fn split(number: Vec<i8>, index: usize) -> Vec<i8> {
    // copy part before index
    let mut result = number[..index].to_vec();
    let mut tail = number[(index + 1)..].to_vec();

    result.push(-1);
    result.push(number[index] / 2);
    result.push(((number[index] as f32) / 2.0).round() as i8);
    result.push(-2);

    // append part after index
    result.append(&mut tail);
    result
}

fn get_magnitude(number: &Vec<i8>) -> usize {
    if number.len() == 4 { return 3 * number[1] as usize + 2 * number[2] as usize; }
    let left;
    let right;

    // for &val in number {
    //     match val {
    //         -2 => print!("]"),
    //         -1 => print!("["),
    //         _ => print!(" {} ", val)
    //     }
    // }
    // println!();

    if number[1] >= 0 {
        left = number[1] as usize;
        right = get_magnitude(&number[2..=(number.len() - 2)].to_vec());
    } else if number[number.len() - 2] >= 0 {
        right = number[number.len() - 2] as usize;
        left = get_magnitude(&number[1..=(number.len() - 3)].to_vec());
    } else {
        let mut middle = 0;
        let mut counter = 0;
        for i in 1..number.len() {
            match number[i] {
                -2 => {
                    counter -= 1;
                    if counter == 0 {
                        middle = i;
                        break;
                    }
                }
                -1 => counter += 1,
                _ => ()
            }
        }
        left = get_magnitude(&number[1..=middle].to_vec());
        right = get_magnitude(&number[(middle + 1)..(number.len() - 1)].to_vec());
    }

    3 * left + 2 * right
}