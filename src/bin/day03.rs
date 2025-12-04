use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = part1();
    return Ok(());
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(
        "problem-inputs/day03.txt",
    )?;
    
    let reader = BufReader::new(file);

    let mut answer = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        
        let mut digit_upper_option: Option<usize> = None;
        let mut digit_lower_option: Option<usize> = None;
        for char in line.chars() {
            let digit: usize = char.to_string().parse()?;

            if digit_upper_option.is_none() && digit_lower_option.is_none() {
                digit_lower_option = Some(digit)
            } else if digit_upper_option.is_none() && let Some(digit_lower) = digit_lower_option {
                digit_upper_option = Some(digit_lower);
                digit_lower_option = Some(digit)
            } else if let Some(digit_upper) = digit_upper_option && let Some(digit_lower) = digit_lower_option {
                if digit_lower > digit_upper {
                    digit_upper_option = Some(digit_lower);
                    digit_lower_option = Some(digit);
                }

                if digit  > digit_lower {
                    digit_lower_option = Some(digit)
                }
            }

        }

        if let Some(digit_upper) = digit_upper_option && let Some(digit_lower) = digit_lower_option {
            answer += digit_upper * 10 + digit_lower;
        }

    }


    println!("[part 1] total joltage: {}", answer);

    return Ok(());
}