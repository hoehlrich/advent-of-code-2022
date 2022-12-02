use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut data: Vec<i32> = fs::read_to_string("input.txt")?
        .split("\n\n")
        .filter_map(|v| v.parse::<String>().ok())
        .map(|g| g.lines().filter_map(|v| v.parse::<i32>().ok()).sum::<i32>())
        .collect();

    data.sort();
    println!("{}", data.last().unwrap());
    println!("{}", data[data.len()-3..].iter().sum::<i32>());
    Ok(())
}
