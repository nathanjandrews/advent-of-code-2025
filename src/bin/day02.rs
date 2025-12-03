use std::{collections::HashSet, fs::File, io::{BufReader, Read}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = part1();
    let _ = part2();
    return Ok(());
}

fn is_invalid_id(n: i64) -> bool {
    // the number of digits in n
    let n_num_digits = count_digits(n);
    let n_string = n.to_string();

    // we now want to construct all of the repeated string that can be formed
    // from these digits. For example, if n is 100, we construct the following
    // numbers to test against:
    // -Y 111 (1) -> 1 is a factor of 3 so we can construct this number and be
    // sure that the sequence repeats
    // -N 101 (10, truncated) -> We do not construct this number because 2 is not
    // a factor of 3, so we know the pattern will not repeat
    // -N 110 (110, original) -> We do not construct this number because there
    // is no repetition because it is the original. If there were repetition,
    // then we would have caught it before and the function would have already
    // returned true.

    // we start with the empty string and incrementally add characters
    let mut segment = "".to_string();
    for char in n_string.chars() {
        segment += &char.to_string();
        if n_num_digits % segment.len() != 0 || segment == n_string {
            // if the number of digits in the segment does not divide in to the
            // number of digits in the original number, then we know that
            // the pattern will not repeat
            continue
        }

        let new_str = segment.repeat(n_num_digits / segment.len());
        if n_string == new_str {
            return true;
        }
    }

    return false;
}

fn count_digits(n: i64) -> usize {
    return n.to_string().len();
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
     let file = File::open(
        "problem-inputs/day02.txt",
    )?;

    let mut reader = BufReader::new(file);

    let mut file_context = String::new();
    match reader.read_to_string(&mut file_context) {
        Ok(_) => (),
        Err(err) => panic!("error converting file content to string: {}", err)
    }

    let mut set = HashSet::<i64>::new();

    let mut answer = 0;
    for range in file_context.split(',') {
        if let Some((min_str, max_str)) = range.split_once("-") &&
           let Ok(min) = min_str.parse::<i64>() &&
           let Ok(max) = max_str.parse::<i64>()
        {
            for cur in min..=max {
                if !is_invalid_id(cur) {
                    continue;
                }

                if cur >= min && cur <= max && !set.contains(&cur) {
                    answer += cur;
                    set.insert(cur);
                }
            }
        } else {
            panic!("Unable to parse segment: {}", range);
        }
    }

    println!("[part 2] sum of invalid ids: {}", answer);

    return Ok(());
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(
        "problem-inputs/day02.txt",
    )?;
    
    let mut reader = BufReader::new(file);

    let mut file_context = String::new();
    match reader.read_to_string(&mut file_context) {
        Ok(_) => (),
        Err(err) => panic!("error converting file content to string: {}", err)
    }

    let mut set = HashSet::<i64>::new();

    let mut answer = 0;
    for range in file_context.split(',') {
        if let Some((min_str, max_str)) = range.split_once("-") &&
           let Ok(min) = min_str.parse::<i64>() && 
           let Ok(max) = max_str.parse::<i64>()
        {
            for cur in min..=max {
                let invalid_id = match get_invalid_id(cur) {
                    None => continue,
                    Some(v) => v,
                };
                

                if invalid_id >= min && invalid_id <= max && !set.contains(&invalid_id) {
                    answer += invalid_id;
                    set.insert(invalid_id);
                }
            }
        } else {
            panic!("Unable to parse segment: {}", range);
        }
    }


    println!("[part 1] sum of invalid ids: {}", answer);

    return Ok(());
}

fn get_invalid_id(n: i64) -> Option<i64> {
    let num_digits = n.to_string().len();
    if num_digits % 2 == 1 {
        return None;
    }

    let n_str = n.to_string();
    let (n_first_half_str, _ ) = n_str.split_at(n_str.len() / 2);
    let n_concat = n_first_half_str.to_owned() + n_first_half_str;
    return match n_concat.parse::<i64>() {
        Err(err) => panic!("unable to parse string into i64: {}", err),
        Ok(value) => Some(value)
    };
}