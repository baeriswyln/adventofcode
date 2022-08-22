mod year2021;

use anyhow::Context;
use humantime::format_duration;
use std::{
    fs,
    time::{Duration, Instant}
};

trait Advent {
    fn new(data: &str) -> Self
        where
            Self: Sized;
    fn year() -> u16
        where
            Self: Sized;
    fn day() -> u8
        where
            Self: Sized;
    fn part1(&self) -> usize;
    fn part2(&self) -> usize;
}

struct Solution {
    event: Box<dyn Advent>,
    time: Duration,
    day: u8,
    year: u16,
}

impl Solution {
    fn new<Event: Advent + 'static>(content: &str) -> Self {
        let (event, time) = get_time(|| Event::new(content));

        Solution {
            event: Box::new(event),
            time,
            day: Event::day(),
            year: Event::year(),
        }
    }

    fn get_result(&self) {
        let (part1, time1) = get_time(|| self.event.part1());
        let (part2, time2) = get_time(|| self.event.part2());

        println!();

        println!(
            "Solution for {} December {}, duration: {}",
            self.day,
            self.year,
            format_duration(self.time)
        );

        println!(
            "part 1: {} in {}",
            part1,
            format_duration(time1),
        );

        println!(
            "part 2: {} in {}",
            part2,
            format_duration(time2),
        );
    }
}

pub fn get_solution(year: u16, day: u8) {
    let filename = format!("src/year{}/day{:0>2}/input.txt", year, day);
    let content = &fs::read_to_string(&filename)
        .with_context(|| format!("count not read input file year{}/day{:0>2}/input.txt", year, day))
        .unwrap();

    let solution = match year {
        2021 => match day {
            1 => Solution::new::<year2021::day01::SonarSweep>(content),
            2 => Solution::new::<year2021::day02::Dive>(content),
            3 => Solution::new::<year2021::day03::BinaryDiagnostic>(content),
            4 => Solution::new::<year2021::day04::Bingo>(content),
            _ => unreachable!(),
        }
        _ => unreachable!(),
    };

    solution.get_result();
}

fn get_time<T>(f: impl FnOnce() -> T) -> (T, Duration){
    let start = Instant::now();
    let result = f();
    let time = start.elapsed();

    (result, time)
}