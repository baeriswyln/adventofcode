enum Direction {
    Up,
    Down,
    Forward,
}

struct Cmd {
    dir: Direction,
    count: usize,
}

pub struct Dive {
    data: Vec<Cmd>,
}

impl crate::Advent for Dive {
    fn new(data: &str) -> Dive {
        let cmds: Vec<Cmd> = data
            .lines()
            .map(|d| {
                let cmd = d.split_whitespace().collect::<Vec<&str>>();
                let dir = match cmd[0] {
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    _ => Direction::Forward,
                };
                let count = cmd[1].parse().unwrap();
                Cmd { dir, count }
            })
            .collect();
        Dive { data: cmds }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 2 }

    fn part1(&self) -> usize {
        let mut depth = 0;
        let mut position = 0;

        for cmd in self.data.iter() {
            match cmd.dir {
                Direction::Down => depth += cmd.count,
                Direction::Up => depth -= cmd.count,
                _ => position += cmd.count,
            }
        }

        (depth * position) as usize
    }

    fn part2(&self) -> usize {
        let mut aim = 0;
        let mut depth = 0;
        let mut position = 0;

        for cmd in self.data.iter() {
            match cmd.dir {
                Direction::Down => aim += cmd.count,
                Direction::Up => aim -= cmd.count,
                _ => {
                    position += cmd.count;
                    depth += aim * cmd.count;
                }
            }
        }

        (depth * position) as usize
    }
}
