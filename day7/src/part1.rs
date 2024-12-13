pub fn run(input: String) {
    let equations = input
        .lines()
        .map(|line| line.split(":").collect::<Vec<&str>>())
        .map(|x|
        (
            x[0].parse::<u128>().unwrap(),
            x[1].split_whitespace().map(|num| num.parse::<u128>().unwrap()).collect::<Vec<u128>>()
        ))
        .collect::<Vec<(u128, Vec<u128>)>>();

    let rizz: u128 = equations
        .iter()
        .filter(|equation| above(equation.0, &*equation.1, 0, 0))
        .map(|equation| equation.0)
        .sum::<u128>();

    println!("{}", rizz)
}

fn above(result: u128, nums: &[u128], num: u128, index: usize) -> bool {
    if index == nums.len() && num == result { return true }
    if index == nums.len() { return false }

    above(result, nums, nums[index] * num.max(1), index + 1)
    || above(result, nums, nums[index] + num, index + 1)
}