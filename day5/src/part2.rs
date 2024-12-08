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
            if let (Some(pos_a), Some(pos_b)) = (update.iter().position(|&x| x == a), update.iter().position(|&x| x == b)) {
                if pos_a > pos_b {
                    is_correct = false;
                    break;
                }
            }
        }

        if !is_correct {
            let mut sorted_update = update.clone();
            sorted_update.sort_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    std::cmp::Ordering::Less
                } else if rules.contains(&(b, a)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });

            let middle_index = sorted_update.len() / 2;
            rizz += sorted_update[middle_index];
        }
    }

    println!("{}", rizz);
}
