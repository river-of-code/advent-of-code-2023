use std::{io::BufReader, io::BufRead, fs::File, collections::HashMap};

pub fn run() -> u32 {
  let reader = BufReader::new(
    File::open("src/day_two/input.txt").unwrap()
  );
  
  reader
  .lines()
  .flat_map(|l| l)
  .map(|l| color_product(l.as_str()))
  .sum()
}

/*
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
 */
fn color_product(line: &str) -> u32 {
  let mut minimum_colors: HashMap<&str, u32>= HashMap::from([
    ("red", 0),
    ("green", 0),
    ("blue", 0),
  ]);
  let game_and_rounds: Vec<&str> = line.split(":").collect();
  assert_eq!(game_and_rounds.len(), 2);
  let raw_rounds = game_and_rounds.get(1).unwrap();
  let rounds: Vec<&str> = raw_rounds.split(";").collect();
  for round in rounds {
    let blocks = round.split(",").map(|r| r.trim());
    for block in blocks {
      let s: Vec<&str> = block.split(" ").collect();

      let count = s.get(0).unwrap().parse::<u32>().expect("this should have been a number");
      let color = s.get(1).unwrap();
      if count > minimum_colors[color] {
        minimum_colors.insert(color, count);
      }
    }
  }
  minimum_colors.values().product()
}