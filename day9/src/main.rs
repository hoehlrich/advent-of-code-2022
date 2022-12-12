use std::error::Error;
use std::fs;

enum Dir {
    R,
    U,
    L,
    D,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut visited = vec![(0, 0)];
    let mut ropes = vec![(0, 0); 10];
    let _ = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            let tokens = l.split_whitespace().collect::<Vec<&str>>();
            let dir = match tokens[0] {
                "R" => Dir::R,
                "U" => Dir::U,
                "L" => Dir::L,
                "D" => Dir::D,
                _ => unreachable!(),
            };
            let n = match tokens[1].parse::<isize>() {
                Ok(v) => v,
                Err(_) => unreachable!(),
            };

            exec_move(dir, n, &mut ropes, &mut visited);
        })
        .collect::<()>();
    println!("{}", visited.len());
    Ok(())
}

fn exec_move(
    dir: Dir,
    n: isize,
    ropes: &mut Vec<(isize, isize)>,
    visited: &mut Vec<(isize, isize)>,
) {
    for _ in 0..n {
        match dir {
            Dir::R => ropes[0].1 += 1,
            Dir::U => ropes[0].0 += 1,
            Dir::L => ropes[0].1 -= 1,
            Dir::D => ropes[0].0 -= 1,
        };
        for i in 1..ropes.len() {
            drag(&mut ropes[i-1].clone(), &mut ropes[i]);
        }
        if !visited.contains(ropes.last().unwrap()) {
            visited.push(ropes.last().unwrap().clone());
        }
    }
}

fn drag(puller: &mut (isize, isize), pulled: &mut (isize, isize)) {
    if puller == pulled {
        return;
    }

    let (ydiff, xdiff) = (
        (puller.0 - pulled.0) as f32 / 2.0,
        (puller.1 - pulled.1) as f32 / 2.0,
    );

    if ydiff.abs() < 1.0 && xdiff.abs() < 1.0 {
        return;
    }

    let (yy, xx) = (
        (ydiff.abs().ceil() * ydiff.signum()) as isize,
        (xdiff.abs().ceil() * xdiff.signum()) as isize,
    );

    pulled.0 += yy;
    pulled.1 += xx;
}
