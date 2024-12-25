use std::collections::HashMap;

pub fn run(input: String) {
    let stones: Vec<u128> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    
    let mut hash_stones: HashMap<u128, u128> = HashMap::new();
    for stone in stones {
        *hash_stones.entry(stone).or_insert(0) += 1
    }
    let rizz: u128 = blink(&hash_stones, 75).values().sum::<u128>();

    println!("{:?}", rizz);
}
fn blink(nums: &HashMap<u128, u128>, depth: usize) -> HashMap<u128, u128> {
    if depth == 0 {
        return nums.clone();
    }

    let mut new_nums: HashMap<u128, u128> = HashMap::new();
    for (&x, &count) in nums.iter() {
        let length = x.to_string().len();
        if x == 0 {
            *new_nums.entry(1).or_insert(0) += count;
        } else if length & 1 == 0 {
            let mid = length / 2;
            let x_str = x.to_string();
            let first_half: u128 = x_str[..mid].parse().unwrap();
            let second_half: u128 = x_str[mid..].parse().unwrap();
            *new_nums.entry(first_half).or_insert(0) += count;
            *new_nums.entry(second_half).or_insert(0) += count;
        } else {
            *new_nums.entry(x * 2024).or_insert(0) += count;
        }
    }

    blink(&new_nums, depth - 1)
}