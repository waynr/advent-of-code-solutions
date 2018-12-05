extern crate clap;
extern crate reqwest;

mod aoc;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use clap::value_t;


fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("aoc2018")
        .version("0.0")
        .author("Wayne Warren")
        .about("Personal CLI for Advent of Code 2018")
        .arg(Arg::with_name("day")
            .help("the advent day")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("part")
            .help("the part (1 or 2)")
            .required(true)
            .index(2)
        )
        .arg(Arg::with_name("session_id")
            .help("the session id cookie value to use to obtain inputs")
            .short("s")
            .long("session")
            .takes_value(true)
            .value_name("SESSION_ID")
            .required(true)
        )
        //.subcommand(SubCommand::with_name("get-input")
        //    .about("obtain input for the given day"))
        //.subcommand(SubCommand::with_name("submit")
        //    .about("obtain input then calculate and submit solution for the given day"))
        .subcommand(SubCommand::with_name("exec")
            .about("obtain input then calculate solution for the given day"))
        .get_matches();

    let day: u8 = value_t!(matches, "day", u8).unwrap_or_else(|e| e.exit());
    let part: u8 = value_t!(matches, "part", u8).unwrap_or_else(|e| e.exit());
    let session_id: String = value_t!(matches, "session_id", String).unwrap_or_else(|e| e.exit());

    let advent_day = aoc::AdventDay::new(2018, day);
    let input = advent_day.get_input(session_id)?;
    //println!("{}", input);
    //

    match (day, part) {
        (1, 1) => aoc::day1solution1(input),
        (1, 2) => aoc::day1solution2(input),
        (2, 1) => aoc::day2solution1(input),
        (_, _) => {
            eprintln!("Invalid day/part combo: day {} part {}", day, part);
            std::process::exit(1);
        }
    };

    Ok(())
}
