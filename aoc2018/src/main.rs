extern crate reqwest;

fn main() -> Result<(), Box<std::error::Error>> {
    let cli = reqwest::Client::new();
    let req = cli
        .get("https://adventofcode.com/2018/day/1/input")
        .header(reqwest::header::COOKIE, "session=53616c7465645f5f1925b812ca83a437cb646c356e8f9061af30c5a7da04361914e658d95444af03c01c482303375958");

    let mut res = req.send()?;

    let freq: i32  = res
        .text()?
        .split('\n')
        .map(|s: &str| s.to_string().parse::<i32>().ok())
        .filter_map(|e| e)
        .sum();

    println!("{}", freq);

    Ok(())
}
