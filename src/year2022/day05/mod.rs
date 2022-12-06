#[derive(Copy, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Clone)]
pub struct SupplyStacks {
    stacks: Vec<Vec<char>>,
    data: Vec<Move>,
}

impl crate::Advent for SupplyStacks {
    fn new(data: &str) -> Self {
        let mut iter = data.split("\n\n");

        // decode the stacks
        let mut stacks = Vec::new();
        let mut stack_iter = iter.next().unwrap().lines().rev();

        // ignore last line and initialize inner vectors
        for _ in 0..((stack_iter.next().unwrap().len() + 2) / 4) {
            stacks.push(Vec::new());
        }

        println!("length: {}", stacks.len());

        // read the lines
        stack_iter.for_each(|l| {
            for i in 0..((l.len() + 1) / 4) {
                let c = l.as_bytes()[(i * 4) + 1] as char;
                if c != ' ' {
                    stacks.get_mut(i).unwrap().push(c);
                }
            }
        });

        // decode moves
        let re = regex::Regex::new(r"move | from | to ").unwrap();
        let data = iter.next().unwrap().lines().map(|l| {
            let mut move_iter = re.split(l);
            move_iter.next(); // jump first
            Move {
                count: move_iter.next().unwrap().parse().unwrap(),
                from: move_iter.next().unwrap().parse().unwrap(),
                to: move_iter.next().unwrap().parse().unwrap(),
            }
        }).collect();

        SupplyStacks { stacks, data }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        5
    }

    fn part1(&self) -> usize {
        let mut stacks = self.stacks.clone();
        self.data.iter().for_each(|m| {
            for _ in 0..m.count {
                let e = stacks.get_mut(m.from - 1).unwrap().pop().unwrap();
                stacks.get_mut(m.to - 1).unwrap().push(e);
            }
        });

        stacks.iter_mut().for_each(|s| print!("{}", s.pop().unwrap()));
        println!();

        0
    }

    fn part2(&self) -> usize {
        let mut stacks = self.stacks.clone();

        self.data.iter().for_each(|m| {
            let mut temp_stack = Vec::new();

            for _ in 0..m.count {
                temp_stack.push(stacks.get_mut(m.from - 1).unwrap().pop().unwrap());
            }
            for _ in 0..m.count {
                stacks.get_mut(m.to - 1).unwrap().push(temp_stack.pop().unwrap());
            }
        });

        stacks.iter_mut().for_each(|s| print!("{}", s.pop().unwrap()));
        println!();

        0
    }
}