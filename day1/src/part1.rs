pub fn run(input: String) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap());

        vec1.push(nums.next().unwrap());
        vec2.push(nums.next().unwrap())
    }

    vec1.sort();
    vec2.sort();

    let rizz = vec1
        .iter()
        .zip(vec2.iter())
        .map(|x| (x.0 - x.1).abs())
        .sum::<i32>();

    println!("{}", rizz)
}