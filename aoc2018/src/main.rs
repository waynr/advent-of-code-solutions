mod aoc;

use std::collections::HashSet;


fn main() -> Result<(), Box<std::error::Error>> {
    let advent_day: aoc::AdventDay = aoc::AdventDay {
        year: 2018,
        day: 1,
    };

    let session_id = String::from("meow");
    let input = advent_day.get_input(session_id)?;
    //println!("{}", input);

    let freq_deltas: Vec<i32> = input
        .split('\n')
        .map(|s: &str| s
            .to_string()
            .parse::<i32>()
            .ok()
        )
        .filter_map(|x| x)
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
