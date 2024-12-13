pub fn run(input: String) {
    let equations = input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let result = parts.next().unwrap().parse::<u128>().unwrap();
            let nums = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u128>().unwrap())
                .collect::<Vec<_>>();
            (result, nums)
        });

    let rizz: u128 = equations
        .filter(|(result, nums)| approve(*result, nums, 0, 0))
        .map(|(result, _)| result)
        .sum();

    println!("{}", rizz)
}

fn approve(result: u128, nums: &[u128], num: u128, index: usize) -> bool {
    if index == nums.len() && num == result { return true }
    if index == nums.len() { return false }

    approve(result, nums, nums[index] * num.max(1), index + 1)
    || approve(result, nums, nums[index] + num, index + 1)
}