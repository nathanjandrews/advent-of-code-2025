use std::{collections::HashSet, fs::File, io::{BufReader, Read}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = part1();
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