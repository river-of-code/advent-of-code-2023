use std::{io::BufReader, io::BufRead, fs::File};

pub fn run() -> u32 {
  let reader = BufReader::new(
    File::open("src/day_one/input.txt").unwrap()
  );
  
  let mut sum = 0u32;

  for line in reader.lines() {
    let parsed_line = line.unwrap();
    let chars = parsed_line.chars();

    let mut digits = chars
              .into_iter()
              .filter(|c| c.is_digit(10));

    let first_digit = digits.next().unwrap();
    let mut last_digit = first_digit;

    while let Some(c) = digits.next() {
      last_digit = c;
    };

    let to_add: String = vec![first_digit, last_digit].into_iter().collect();

    print!("adding: {}", to_add);
    
    sum += to_add.parse::<u32>().unwrap_or(0);

    println!(", sum is now: {}", sum);
  }

  sum
}