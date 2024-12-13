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

fn approve(result: u128, nums: &[u128], current: u128, index: usize) -> bool {
    if index == nums.len() {
        return current == result;
    }

    approve(result, nums, nums[index] * current.max(1), index + 1)
    || approve(result, nums, nums[index] + current, index + 1)
}