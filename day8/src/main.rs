mod part1;
mod part2;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    part1::run(input.clone());
    part2::run(input);
}
