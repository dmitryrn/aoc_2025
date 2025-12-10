use std::env;
use std::fs;
use std::path::Path;

pub fn fetch_input(day: u32) -> Result<String, Box<dyn std::error::Error>> {
    let path = format!("inputs/{}.txt", day);
    if Path::new(&path).exists() {
        return Ok(fs::read_to_string(&path)?);
    }
    let session = env::var("AOC_SESSION")?;
    let url = format!("https://adventofcode.com/2025/day/{}/input", day);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch: {}", response.status()).into());
    }
    let text = response.text()?;
    fs::create_dir_all("inputs")?;
    fs::write(&path, &text)?;
    Ok(text)
}