pub fn run(input: String) {
    let mut safe_count = 0;
    for line in input.lines() {

        let report: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if is_safe(&report) {
            safe_count += 1;
        } else {
            for i in 0..report.len() {
                let mut report_copy = report.clone();
                report_copy.remove(i);
                if is_safe(&report_copy) {
                    safe_count += 1;
                    break;
                }
            }
        }

    }
    println!("{}", safe_count);
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let first_diff = report[1] - report[0];

    if first_diff == 0 || first_diff.abs() > 3 {
        return false;
    }

    let expected_sign = first_diff.signum();

    for i in 1..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        let sign = diff.signum();
        if sign != expected_sign {
            return false;
        }
    }
    true
}