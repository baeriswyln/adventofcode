struct Vector {
    x: isize,
    y: isize,
}

struct Probe {
    pos: Vector,
    vec: Vector,
    max_y: isize,
}

impl Probe {
    fn new(initial_vector: Vector) -> Self {
        Probe {
            pos: Vector { x: 0, y: 0 },
            vec: initial_vector,
            max_y: isize::MIN,
        }
    }

    fn step(&mut self) {
        self.pos.x += self.vec.x;
        self.pos.y += self.vec.y;
        self.vec.x += if self.vec.x > 0 { -1 } else if self.vec.x < 0 { 1 } else { 0 };
        self.vec.y -= 1;
        if self.pos.y > self.max_y { self.max_y = self.pos.y }
    }

    fn is_inside(&self, target_area: &TrickShot) -> bool {
        self.pos.x >= target_area.x_min &&
            self.pos.x <= target_area.x_max &&
            self.pos.y >= target_area.y_min &&
            self.pos.y <= target_area.y_max
    }

    fn has_passed(&self, target_area: &TrickShot) -> bool {
        self.pos.x > target_area.x_max || self.pos.y < target_area.y_min
    }
}

pub struct TrickShot {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl crate::Advent for TrickShot {
    fn new(data: &str) -> Self {
        let x = data.split("x=").nth(1).unwrap().split(",").next().unwrap();
        let y = data.split("y=").nth(1).unwrap();
        TrickShot {
            x_min: x.split("..").nth(0).unwrap().parse().unwrap(),
            x_max: x.split("..").nth(1).unwrap().parse().unwrap(),
            y_min: y.split("..").nth(0).unwrap().parse().unwrap(),
            y_max: y.split("..").nth(1).unwrap().parse().unwrap(),
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 17 }

    fn part1(&self) -> usize {
        // if the initial velocity y > 0 , the probe will always appear at point y=0 falling down
        // with the negative initial velocity. therefore, this velocity has to be < y_min of the
        // target field. otherwise, the probe will always overshoot the target.
        // abs(y_min)-1 is therefore the highest possible initial velocity. and the maximum
        // height is simply the sum of all numbers from 0 to abs(y_min)-1.
        let n = (self.y_min.abs() - 1) as usize;
        n * (n + 1 ) / 2
    }

    fn part2(&self) -> usize {
        let mut counter = 0;

        for x in 0..=self.x_max {
            'a: for y in self.y_min..self.y_min.abs() {
                let mut probe = Probe::new(Vector { x, y });

                while !probe.has_passed(&self) {
                    probe.step();
                    if probe.is_inside(&self) {
                        counter += 1;
                        continue 'a;
                    }
                }
            }
        }

        counter
    }
}