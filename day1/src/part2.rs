pub fn run(input: String) {
    let mut vec1 = Vec::new();
    let mut counter2 = std::collections::HashMap::new();

    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap());

        vec1.push(nums.next().unwrap());
        *counter2.entry(nums.next().unwrap()).or_insert(0) += 1
    }

    let mut rizz = 0;
    for num in vec1.iter() {
        rizz += num * counter2.get(num).unwrap_or(&0)
    }
    println!("{}", rizz)
}