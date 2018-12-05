extern crate reqwest;

use std::collections::HashSet;


pub struct AdventDay {
    year: u16,
    day: u8,
}

impl AdventDay {
    pub fn new(year: u16, day: u8) -> AdventDay {
        AdventDay {
            year: year,
            day: day
        }
    }

    pub fn get_input(
        &self,
        session_id: String
    ) ->  Result<String, Box<std::error::Error>>
    {
        let cli = reqwest::Client::new();
        let url = format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day);
        let cookie = format!("session={}", session_id);

        //println!("{}", url);
        //println!("{}", session_id);
        let req = cli.get(&url)
            .header(reqwest::header::COOKIE, cookie);
        //println!("{:?}", req);
        let mut res = req.send()?;

        //println!("{:?}", res);
        //println!("{}", res.text()?);
        Ok(res.text()?.to_string())
    }
}

fn parse_day1_input(input: String) -> Vec<i32> {
    input.split('\n')
        .map(|s: &str| s
            .to_string()
            .parse::<i32>()
            .ok()
        )
        .filter_map(|x| x)
        .collect()
}

pub fn day1solution1(input: String) {
    let freq_deltas = parse_day1_input(input);

    let freq: i32 = freq_deltas
        .iter()
        .sum();

    println!("day 1 solution 1: {}", freq);
}

pub fn day1solution2(input: String) {
    let freq_deltas = parse_day1_input(input);

    let mut delta_cycle = freq_deltas
        .iter()
        .cycle();

    let mut freqs = HashSet::new();
    let mut current_freq: i32 = 0;
    freqs.insert(current_freq);

    loop {
        match delta_cycle.next() {
            Some(d) => current_freq += d,
            None => break,
        }

        if freqs.contains(&current_freq) {
            break
        }

        freqs.insert(current_freq);
    }
    println!("day 1 solution 2: {}", current_freq);
}
