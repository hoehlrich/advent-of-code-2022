use std::error::Error;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("input.txt")?
        .lines()
        .next()
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?
        .parse::<String>()?;

    const LENGTH: usize = 14;
    let mut prev = [' '; LENGTH];
    let mut chars = data.chars().enumerate();
    for i in 0..LENGTH-1 {
        prev[LENGTH-2-i] = chars
            .next()
            .ok_or(io::Error::from(io::ErrorKind::InvalidData))?
            .1;
    }

    'outer: for (i, c) in data.chars().enumerate().skip(3) {
        prev.rotate_right(1);
        prev[0] = c;
        for (j, pc) in prev.iter().enumerate() {
            for (k, pcc) in prev.iter().enumerate() {
                if pc == pcc && j != k {
                    continue 'outer;
                }
            }
        }
        println!("{} - {}", i + 1, c);
        break;
    }
    Ok(())
}
