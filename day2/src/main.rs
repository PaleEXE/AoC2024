use std::fs;
mod part1;
mod part2;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1::run(input.clone());
    part2::run(input);
}
