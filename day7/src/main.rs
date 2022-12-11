use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut path = vec![];
    let mut curr_dir = String::new();
    let mut total_used = 0;

    let _ = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            let tokens = l.split_whitespace().collect::<Vec<&str>>();

            match tokens[0] {
                "$" => {
                    match tokens[1] {
                        "cd" => match tokens[2] {
                            ".." => _ = path.pop().unwrap(),
                            v => {
                                path.push(v.to_string());
                                curr_dir = path.join("-");
                                dirs.insert(curr_dir.clone(), 0);
                            }
                        },
                        _ => (),
                    }
                }
                v => match v.parse::<u32>() {
                    Ok(n) => {
                        total_used += n;
                        for i in 1..path.len() {
                            let pathstr = path[0..i+1].join("-");
                            dirs.entry(pathstr.clone()).and_modify(|e| *e += n);
                        }
                    },
                    Err(_) => (),
                }
            }
        })
        .collect::<()>();

    let mut sum = 0;
    let required_removed = 30000000 - (70000000 - total_used);
    let mut smallest_good: u32 = 4000000000;
    for v in dirs.values() {
        if v <= &100000 {
            sum += v;
        } else if v >= &required_removed && v < &smallest_good {
            smallest_good = *v;
        }
    }

    println!("{}", sum);
    println!("{:?}", smallest_good);

    Ok(())
}
