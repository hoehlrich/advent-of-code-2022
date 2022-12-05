use std::collections::HashMap;
use std::error;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn error::Error>> {
    let binding = fs::read_to_string("input.txt")?;
    let (crates_data, instructions_data) = binding
        .split_once("\n\n")
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;

    let mut crates: HashMap<usize, Vec<char>> = HashMap::new();

    let _ = crates_data
        .lines()
        .rev()
        .map(|l| {
            let mut chars = l.chars();
            chars.next();
            let _ = chars
                .step_by(4)
                .enumerate()
                .map(|(i, c)| {
                    if c.is_ascii_uppercase() {
                        if crates.contains_key(&(i + 1)) {
                            crates.entry(i + 1).and_modify(|e| e.push(c));
                        } else {
                            crates.insert(i + 1, vec![c]);
                        }
                    }
                })
                .collect::<()>();
        })
        .collect::<()>();

    let mut pt_one = crates.clone();

    for line in instructions_data.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let n = tokens[1].parse::<usize>()?;
        let from = tokens[3].parse::<usize>()?;
        let to = tokens[5].parse::<usize>()?;

        for _i in 0..n {
            let mut pop = ' ';
            pt_one.entry(from).and_modify(|e| {
                pop = e.pop().unwrap();
            });
            pt_one.entry(to).and_modify(|ee| ee.push(pop));
        }
    }

    let mut pt_two = crates.clone();

    for line in instructions_data.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let n = tokens[1].parse::<usize>()?;
        let from = tokens[3].parse::<usize>()?;
        let to = tokens[5].parse::<usize>()?;

        let mut pops = vec![];
        for _ in 0..n {
            pt_two.entry(from).and_modify(|e| {
                pops.push(e.pop().unwrap());
            });
        }
        for _ in 0..n {
            pt_two
                .entry(to)
                .and_modify(|ee| ee.push(pops.pop().unwrap()));
        }
    }

    for k in pt_two.keys() {
        println!("{} -> {:?}", k, pt_two[k]);
    }

    Ok(())
}
