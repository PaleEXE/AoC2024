pub fn run(input: String) {
    let mut rizz = 0;
    let mut chars = input.chars().collect::<Vec<_>>();
    let mut doable = true;

    for i in 0..chars.len() {
        if i + 3 < chars.len() && &chars[i..i + 4] == ['d', 'o', '(', ')'] { doable = true}
        if i + 6 < chars.len() && &chars[i..i + 7] == ['d', 'o', 'n', '\'', 't', '(', ')'] {doable = false}
        if i + 3 < chars.len() && &chars[i..i + 4] == ['m', 'u', 'l', '('] && doable {
            if let Some(j) = chars[i + 4..].iter().position(|&c| c == ')') {
                let j = i + 4 + j;
                let para: Vec<String> = chars[i + 4..j]
                    .iter()
                    .collect::<String>()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();

                if para.len() == 2 {
                    if let (Ok(lift), Ok(right)) = (para[0].parse::<i32>(), para[1].parse::<i32>()) {
                        rizz += lift * right;
                    }
                }
            }
        }
    }

    println!("{}", rizz);
}