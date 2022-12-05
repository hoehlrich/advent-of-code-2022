use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let pairs: Vec<[[u32; 2]; 2]> = fs::read_to_string("input.txt")?
        .lines()
        .filter_map(|l| {
            l.split(",")
                .filter_map(|p| {
                    p.split("-")
                        .filter_map(|v| v.parse::<u32>().ok())
                        .collect::<Vec<u32>>()
                        .try_into()
                        .ok()
                })
                .collect::<Vec<[u32; 2]>>()
                .try_into()
                .ok()
        })
        .collect();

    let pt_one = pairs
        .iter()
        .map(|pair| {
            if pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1] {
                1
            } else if pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][1] {
                1
            } else {
                0
            }
        })
        .sum::<u32>();

    let pt_two = pairs
        .iter()
        .map(|pair| {
            for n in pair[0][0]..pair[0][1] + 1 {
                if (pair[1][0]..pair[1][1]+1).contains(&n) {
                    return 1
                }
            }
            0
        })
        .sum::<u32>();

    println!("{}\n{}", pt_one, pt_two);
    Ok(())
}
