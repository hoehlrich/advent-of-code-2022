use std::error::Error;
use std::fs;

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() -> Result<(), Box<dyn Error>> {
    let map = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut c = 0;
    let mut best_score = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_hidden(y, x, &map) {
                c += 1;
            }
            let score = get_scenic_score(y, x, &map);
            if score > best_score {
                best_score = score;
            }
        }
    }
    println!("{}", c);
    println!("{}", best_score);
    Ok(())
}

fn is_hidden(y: usize, x: usize, map: &Vec<Vec<u32>>) -> bool {
    let v = map[y][x];

    for (yy, xx) in NEIGHBORS {
        let mut good = true;
        let (mut y, mut x) = (y, x);
        loop {
            y = ((y as isize) + yy) as usize;
            x = ((x as isize) + xx) as usize;
            match map.get(y) {
                Some(r) => match r.get(x) {
                    Some(c) => if c >= &v {
                        good = false;
                    },
                    None => break,
                },
                None => break,
            }
        }
        if good {
            return true;
        }
    }
    false
}

fn get_scenic_score(y: usize, x: usize, map: &Vec<Vec<u32>>) -> u32 {
    let v = map[y][x];
    let mut dists = vec![];
    for (yy, xx) in NEIGHBORS {
        let mut dist = 0;
        let (mut y, mut x) = (y, x);
        loop {
            y = ((y as isize) + yy) as usize;
            x = ((x as isize) + xx) as usize;
            match map.get(y) {
                Some(r) => match r.get(x) {
                    Some(c) => if c < &v {
                        dist += 1;
                    } else {
                        dist += 1;
                        break;
                    },
                    None => break,
                },
                None => break,
            }
        }
        dists.push(dist);
    }
    let mut score = dists.pop().unwrap();
    for dist in dists {
        score *= dist;
    }
    score
}

