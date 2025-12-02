use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = part1();
    let _ = part2();
    return Ok(());
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(
        "problem-inputs/day01.txt",
    )?;
    let reader = BufReader::new(file);

    let mut answer = 0;
    let mut position = 50;
    for line_result in reader.lines() {
        let line = line_result?;
        if let Some(direction) = line.chars().nth(0) && let Ok(mut magnitude) = line[1..].parse::<i32>()  {
            answer += magnitude / 100;
            
            let start_position = position;

            if direction == 'L' {
                magnitude = magnitude * -1;
            }
            
            position = position + magnitude;
            
            position = position % 100;
            if position < 0 {
                position = 100 + position
            }
            
            if (direction == 'L' && start_position < position && start_position != 0) || (direction == 'R' && start_position > position && start_position != 0) || (position == 0) {
                answer += 1;
            }
        };
    }

    println!("[part 2] num times passed or pointing at zero: {}", answer);

    return Ok(());
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(
        "problem-inputs/day01.txt",
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
