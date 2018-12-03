extern crate reqwest;


pub struct AdventDay {
    pub year: u16,
    pub day: u8,
}

impl AdventDay {
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
