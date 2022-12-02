use std::error::Error;
use std::fs;

#[derive(PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Ending {
    Lose,
    Tie,
    Win,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data: Vec<(Move, Move, Ending)> = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            let mut t = l.split(' ');
            let opponent = match t.next() {
                Some("A") => Move::Rock,
                Some("B") => Move::Paper,
                Some("C") => Move::Scissors,
                _ => unreachable!(),
            };
            let (you, ending) = match t.next() {
                Some("X") => (Move::Rock, Ending::Lose),
                Some("Y") => (Move::Paper, Ending::Tie),
                Some("Z") => (Move::Scissors, Ending::Win),
                _ => unreachable!(),
            };
            (opponent, you, ending)
        })
        .collect();

    let moves = [Move::Rock, Move::Paper, Move::Scissors];

    let pt_one = data
        .iter()
        .map(|v| {
            let i = moves.iter().position(|x| *x == v.1).unwrap();
            let mut total: u32 = 0;
            total += if v.0 == moves[i] {
                3
            } else if v.0 == moves[(i + 2) % 3] {
                6
            } else {
                0
            };
            total += i as u32 + 1;
            total
        })
        .sum::<u32>();

    let pt_two = data
        .iter()
        .map(|v| {
            let i = moves.iter().position(|x| *x == v.0).unwrap();
            let mut total: u32 = 0;
            total += match v.2 {
                Ending::Lose => (i + 2) as u32 % 3,
                Ending::Tie => i as u32 + 3,
                Ending::Win => (i + 1) as u32 % 3 + 6,
            };
            total += 1;
            total
        })
        .sum::<u32>();

    println!("{}\n{}", pt_one, pt_two);
    Ok(())
}
