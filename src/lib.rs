use std::{
    fs,
    time::{Duration, Instant},
};

use anyhow::Context;
use humantime::format_duration;

mod year2021;
mod year2022;

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
            4 => Solution::new::<year2021::day04::GiantSquid>(content),
            5 => Solution::new::<year2021::day05::HydrothermalAdventure>(content),
            6 => Solution::new::<year2021::day06::Lanternfish>(content),
            7 => Solution::new::<year2021::day07::TreacheryOfWhales>(content),
            8 => Solution::new::<year2021::day08::SevenSegment>(content),
            9 => Solution::new::<year2021::day09::SmokeBasin>(content),
            10 => Solution::new::<year2021::day10::SyntaxScoring>(content),
            11 => Solution::new::<year2021::day11::Dumbo>(content),
            12 => Solution::new::<year2021::day12::PassagePathing>(content),
            13 => Solution::new::<year2021::day13::Origami>(content),
            14 => Solution::new::<year2021::day14::Polymerization>(content),
            15 => Solution::new::<year2021::day15::Chiton>(content),
            16 => Solution::new::<year2021::day16::PacketDecoder>(content),
            17 => Solution::new::<year2021::day17::TrickShot>(content),
            18 => Solution::new::<year2021::day18::Snailfish>(content),
            19 => Solution::new::<year2021::day19::BeaconScanner>(content),
            20 => Solution::new::<year2021::day20::TrenchMap>(content),
            21 => Solution::new::<year2021::day21::DiracDice>(content),
            22 => Solution::new::<year2021::day22::ReactorReboot>(content),
            23 => Solution::new::<year2021::day23::Amphipod>(content),
            24 => Solution::new::<year2021::day24::ALU>(content),
            25 => Solution::new::<year2021::day25::Cucumber>(content),
            _ => unreachable!(),
        },
        2022 => match day{
            1 => Solution::new::<year2022::day01::Calories>(content),
            2 => Solution::new::<year2022::day02::RockPaperScissor>(content),
            3 => Solution::new::<year2022::day03::RucksackReorganization>(content),
            4 => Solution::new::<year2022::day04::CampCleanup>(content),
            5 => Solution::new::<year2022::day05::SupplyStacks>(content),
            6 => Solution::new::<year2022::day06::TuningTrouble>(content),
            7 => Solution::new::<year2022::day07::OutOfSpace>(content),
            8 => Solution::new::<year2022::day08::TreeHouse>(content),
            9 => Solution::new::<year2022::day09::RopeBridge>(content),
            10 => Solution::new::<year2022::day10::CathodeRayTube>(content),
            11 => Solution::new::<year2022::day11::Monkey>(content),
            12 => Solution::new::<year2022::day12::HillClimbing>(content),
            13 => Solution::new::<year2022::day13::DistressSignal>(content),
            14 => Solution::new::<year2022::day14::RegolithReservoir>(content),
            15 => Solution::new::<year2022::day15::BeaconExclusionZone>(content),
            16 => Solution::new::<year2022::day16::ProboscideaVolcanium>(content),
            _ => unreachable!()
        }
        _ => unreachable!(),
    };

    solution.get_result();
}

fn get_time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let time = start.elapsed();

    (result, time)
}