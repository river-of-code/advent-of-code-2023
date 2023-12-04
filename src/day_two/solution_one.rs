use std::{io::BufReader, io::BufRead, fs::File, collections::HashMap};

struct Game {
  number: u32,
  valid: bool
}

struct RGB {
  r: u32,
  g: u32,
  b: u32
}

const MAXES: [(&str, u32); 3] = [
  ("red", 12),
  ("green", 13),
  ("blue", 14),
];

pub fn run() -> u32 {
  let reader = BufReader::new(
    File::open("src/day_two/input.txt").unwrap()
  );
  
  reader
  .lines()
  .map(|l| parse_into_game(l.unwrap()))
  .filter(|g| g.valid )
  .fold(0, |acc, g| acc + g.number)
}

fn parse_into_game(line: String) -> Game {
  let a: Vec<&str> = line.split(":").collect();
  assert_eq!(a.len(), 2);
  let b: Vec<&str> = a.get(0).unwrap().split(" ").collect();
  let number = b.get(1).unwrap().parse::<u32>().expect("this should have been a number");
  let valid = is_game_valid(a.get(1).unwrap());
  Game { number, valid }
}

fn is_game_valid(raw_rounds: &str) -> bool {
  let rounds = raw_rounds.split(";").map(|r| r.trim());
  rounds.map(colors_valid).all(|a| a)
}

fn colors_valid(raw_round: &str) -> bool {
  let blocks = raw_round.split(",").map(|r| r.trim());
  for block in blocks {
    let s: Vec<&str> = block.split(" ").collect();
    assert_eq!(s.len(), 2);

    if s.get(0).unwrap().parse::<u32>().expect("this should have been a number") 
        > *MAXES.into_iter().collect::<HashMap<&str, u32>>().get(s.get(1).unwrap()).unwrap() {
      return false
    }
  }
  true
}