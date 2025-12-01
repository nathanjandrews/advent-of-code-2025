use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(
        "problem-inputs/day01-1.txt",
    )?;
    let reader = BufReader::new(file);

    let mut answer = 0;
    let mut position = 50;
    for line_result in reader.lines() {
        let line = line_result?;
       if let Some(direction) = line.chars().nth(0) && let Ok(mut magnitude) = line[1..].parse::<i32>()  {
            if direction == 'L' {
                magnitude = magnitude * -1;
            }

            position = position + magnitude;
            position = position % 100;
            if position == 0 {
                answer += 1
            }
        }
    }

    println!("[part 1] num times at zero: {}", answer);

    return Ok(());
}
