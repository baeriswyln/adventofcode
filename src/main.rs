use adventofcode::get_solution;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "AOC year", default_value = "2021")]
    year: u16,

    #[structopt(help = "AOC day", default_value = "1")]
    day: u8,

    #[structopt(short, long, help = "Show all results")]
    all: bool,
}

fn main() {
    let day_count: u8 = 18;
    let year_first: u16 = 2021;
    let year_count: u16 = 1;
    let args = Cli::from_args();

    if args.all {
        (year_first..year_first + year_count)
            .for_each(
                |y| (1..=day_count).for_each(|d| get_solution(y, d))
            );
    } else {
        get_solution(args.year, args.day);
    }
}