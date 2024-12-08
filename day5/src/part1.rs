pub fn run(input: String) {
    let mut sections = input.split('.');

    let rules = sections
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.trim().split('|');
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>();

    let updates = sections
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut rizz = 0;

    for update in &updates {
        let mut is_correct = true;

        for &(a, b) in &rules {
            if let (Some(pos_a), Some(pos_b)) = (update.iter().position(|&x| x == a),
                                                 update.iter().position(|&x| x == b))
            {
                if pos_a > pos_b {
                    is_correct = false;
                    break;
                }
            }
        }

        let middle_index = update.len() / 2;
        rizz += update[middle_index] * is_correct as i32;

    }

    println!("{}", rizz);
}
