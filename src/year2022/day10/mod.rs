enum Instruction {
    Noop,
    Addx(isize),
}

pub struct CathodeRayTube {
    instructions: Vec<Instruction>,
}

impl crate::Advent for CathodeRayTube {
    fn new(data: &str) -> Self {
        CathodeRayTube {
            instructions: data.lines().map(|l| {
                let mut iter = l.split(" ");
                match iter.next().unwrap() {
                    "addx" => Instruction::Addx(iter.next().unwrap().parse().unwrap()),
                    _ => Instruction::Noop,
                }
            }).collect()
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        10
    }

    fn part1(&self) -> usize {
        let mut x = 1;
        let mut sum = 0;
        let mut index = 0;

        for instr in &self.instructions {
            index += 1;
            if (index - 20) % 40 == 0 { sum += x * index }

            match instr {
                Instruction::Noop => (),
                Instruction::Addx(v) => {
                    index += 1;
                    if (index - 20) % 40 == 0 { sum += x * index }
                    x += v;
                }
            }
        }

        sum as usize
    }

    fn part2(&self) -> usize {
        let mut x = 1;
        let mut index = 0;

        let mut screen = vec![vec![false; 40]; 6];

        for instr in &self.instructions {
            if (index % 40) >= (x - 1) as usize && (index % 40) <= (x + 1) as usize {
                screen[index / 40][index % 40] = true;
            }
            index += 1;

            match instr {
                Instruction::Noop => (),
                Instruction::Addx(v) => {
                    if (index % 40) >= (x - 1) as usize && (index % 40) <= (x + 1) as usize {
                        screen[index / 40][index % 40] = true;
                    }
                    index += 1;
                    x += v;
                }
            }
        }

        for l in screen {
            for v in l {
                if v { print!("#") } else { print!(".") }
            }
            println!()
        }

        0
    }
}
