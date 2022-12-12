use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut x: i32 = 1;
    let mut sum = 0;
    let mut clock_count = 1;
    let mut i: usize = 0;
    let mut j = 0;
    let mut screen = String::new();
    let keyframes = vec![20, 60, 100, 140, 180, 220];

    let binding = fs::read_to_string("input.txt")?;
    let instructions = binding.lines().collect::<Vec<&str>>();
    let mut stack = vec![];

    loop {
        if stack.is_empty() {
            let tokens = instructions[j % instructions.len()]
                .split_whitespace()
                .collect::<Vec<&str>>();

            stack.push(None);
            match tokens.get(1) {
                Some(v) => stack.push(Some(v.parse::<i32>()?)),
                None => (),
            };
            j += 1;
        }

        if (x - (i % 40) as i32).abs() <= 1 {
            screen.push('X');
        } else {
            screen.push(' ');
        }

        if keyframes.contains(&clock_count) {
            sum += x * clock_count;
        }

        match stack.remove(0) {
            Some(v) => x += v,
            None => (),
        };

        if clock_count == 240 {
            break;
        }

        clock_count += 1;
        i += 1;
    }
    println!("{}", sum);
    for i in 0..6 {
        println!("{}", &screen[40*i..40*(i+1)]);
    }
    Ok(())
}
