use std::{io::BufReader, io::BufRead, fs::File, collections::HashMap};

const WORDS: [&'static str; 18] = [
  "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

pub fn run() -> u32 {
  let reader = BufReader::new(
    File::open("src/day_one/input.txt").unwrap()
  );
  
  let mut sum = 0u32;

  for line in reader.lines() {
    let parsed_line = line.unwrap();

    let matches: HashMap<usize, &str> = WORDS
                  .into_iter()
                  .flat_map(|word| parsed_line.match_indices(word))
                  .collect();

    let first_digit_i = matches.keys().min().unwrap();
    let last_digit_i = matches.keys().max().unwrap_or(first_digit_i);

    let to_add: String = vec![
      word_to_num(matches[first_digit_i]),
      word_to_num(matches[last_digit_i]),
    ].into_iter().collect();

    sum += to_add.parse::<u32>().unwrap_or(0);

  }

  sum
}

fn word_to_num(w: &str) -> char {
  match w {
    "one" | "1" => '1',
    "two" | "2" => '2',
    "three" | "3" => '3',
    "four" | "4" => '4',
    "five" | "5" => '5',
    "six" | "6" => '6',
    "seven" | "7" => '7',
    "eight" | "8" => '8',
    "nine" | "9" => '9',
    _ => '0', 
  }

}