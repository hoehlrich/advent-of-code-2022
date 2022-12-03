use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data: Vec<(Vec<u32>, Vec<u32>)> = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            let t = l.split_at(l.len() / 2);
            (
                t.0.chars().map(|v| to_priority(v)).collect(),
                t.1.chars().map(|v| to_priority(v)).collect(),
            )
        })
        .collect();

    let mut dups = vec![];
    for pair in &data {
        'first: for c1 in &pair.0 {
            for c2 in &pair.1 {
                if c1 == c2 {
                    dups.push(c1.clone());
                    break 'first;
                }
            }
        }
    }

    println!("{}", dups.iter().sum::<u32>());

    let data = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.chars().map(|c| to_priority(c)).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut dups = vec![];
    for i in (0..data.len()).step_by(3) {
        'first: for c in &data[i] {
            for cc in &data[i + 1] {
                for ccc in &data[i + 2] {
                    if c == cc && cc == ccc {
                        dups.push(c.clone());
                        break 'first;
                    }
                }
            }
        }
    }

    println!("{}", dups.iter().sum::<u32>());

    Ok(())
}

fn to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}
