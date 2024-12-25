pub fn run(input: String) {
    let mut stones: Vec<String> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.to_owned())
        .collect::<Vec<String>>();

    for _ in 0..25 {
        let stones_length: usize = stones.len();
        let mut new_stones: Vec<String> = Vec::new();
        for index in 0..stones_length {
            if stones[index].len() & 1 == 0 {
                let mid: usize = stones[index].len() / 2;
                let first_half: &str = &stones[index][..mid];
                let second_half: &str = stones[index][mid..].trim_start_matches('0');
                new_stones.push(first_half.to_string());
                if second_half.is_empty() {
                    new_stones.push("0".to_string());
                } else {
                    new_stones.push(second_half.to_string());
                }
            } else if stones[index] == "0" {
                new_stones.push("1".to_string());
            } else {
                match stones[index].parse::<u128>() {
                    Ok(num) => {
                        let x = (num * 2024).to_string();
                        new_stones.push(x);
                    }
                    Err(_) => new_stones.push(stones[index].to_string()),
                }
            }
        }
        stones = new_stones;
    }

    println!("{:?}", stones.len());
}
