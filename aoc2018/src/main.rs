extern crate reqwest;

use std::collections::HashSet;

struct AdventDay {
    year: u16,
    day: u8,
}

fn get_input(day: AdventDay, session_id: String) ->  Result<String, Box<std::error::Error>> {
    let cli = reqwest::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", day.year, day.day);
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

fn main() -> Result<(), Box<std::error::Error>> {
    let advent_day: AdventDay = AdventDay {
        year: 2018,
        day: 1,
    };

    let session_id = String::from("meow");
    let input = get_input(advent_day, session_id)?;
    //println!("{}", input);

    let freq_deltas: Vec<i32> = input
        .split('\n')
        .map(|s: &str| s.to_string().parse::<i32>().ok())
        .filter_map(|e| e)
        .collect();

    let freq: i32 = freq_deltas
        .iter()
        .sum();

    println!("day 1 solution 1: {}", freq);

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

    Ok(())
}
