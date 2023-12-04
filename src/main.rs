pub mod day_one;
pub mod day_two;

fn main() {
    println!("Day 1, Solution 1: {}", day_one::solution_one::run());
    println!("Day 1, Solution 2: {}", day_one::solution_two::run());

    println!("Day 2, Solution 1: {:?}", day_two::solution_one::run());
    // println!("Day 2, Solution 2: {}", day_two::solution_two::run());
}
