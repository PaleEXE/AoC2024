pub fn run(input: String) {
    let mut count = 0;

    let grid: Vec<&str> = input.lines().collect();
    let rows = grid.len();
    let cols = grid[0].len();

    for line in &grid {
        count += line.as_bytes().windows(4).filter(|word| word == b"XMAS" || word == b"SAMX").count();
    }

    for col in 0..cols {
        let column: String = (0..rows).map(|r| grid[r].chars().nth(col).unwrap()).collect();
        count += column.as_bytes().windows(4).filter(|word| word == b"XMAS" || word == b"SAMX").count();
    }

    let directions = [(1, 1), (-1, -1), (1, -1), (-1, 1)];

    for r in 0..rows {
        for c in 0..cols {
            for (dr, dc) in directions.iter() {
                if check_word(&grid, "XMAS", r as isize, c as isize, *dr, *dc) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn check_word(
    grid: &[&str],
    word: &str,
    start_row: isize,
    start_col: isize,
    dr: isize,
    dc: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for (i, ch) in word.chars().enumerate() {
        let nr = start_row + i as isize * dr;
        let nc = start_col + i as isize * dc;

        if nr < 0 || nr >= rows || nc < 0 || nc >= cols || grid[nr as usize].chars().nth(nc as usize).unwrap() != ch {
            return false;
        }
    }
    true
}

