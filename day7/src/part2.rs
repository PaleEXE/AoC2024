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
        })
        .collect::<Vec<_>>();

    let rizz: u128 = equations
        .iter()
        .filter(|(result, nums)| approve(*result, nums, 0, 0))
        .map(|(result, _)| *result)
        .sum();

    println!("{}", rizz);
}

fn approve(result: u128, nums: &[u128], current: u128, index: usize) -> bool {
    if index == nums.len() {
        return current == result;
    }

    let next_num = nums[index];
    let mut concatenated = current;

    approve(result, nums, current.max(1) * next_num, index + 1)
    || approve(result, nums, current + next_num, index + 1)
    || (concatenate(&mut concatenated, next_num)
        && approve(result, nums, concatenated, index + 1))
}

fn concatenate(current: &mut u128, next_num: u128) -> bool {
    let original = *current;
    if let Ok(concat) = format!("{}{}", current, next_num).parse::<u128>() {
        *current = concat;
        true
    } else {
        *current = original;
        false
    }
}
