use std::{fs::File, io::{self, BufRead, BufReader}};

pub fn read_input_file(path: &str) -> Result<Vec<String>, io::Error> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  return reader.lines().collect();
}