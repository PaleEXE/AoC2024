pub fn run(input: String) {
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut direction: u8 = 0;
    let (mut i, mut j) = find_start(&grid).expect("Starting position (^) not found in grid!");

    grid[i][j] = b'X';

    loop {
        if !move_in_direction(&mut grid, &mut i, &mut j, &mut direction) {
            break;
        }
    }

    let rizz = count_visited(&grid);
    println!("{}", rizz);
}

fn find_start(grid: &[Vec<u8>]) -> Option<(usize, usize)> {
    for (row_idx, row) in grid.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&ch| ch == b'^') {
            return Some((row_idx, col_idx));
        }
    }
    None
}

fn move_in_direction(
    grid: &mut [Vec<u8>],
    i: &mut usize,
    j: &mut usize,
    direction: &mut u8,
) -> bool {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (di, dj) = directions[*direction as usize];

    while let Some((ni, nj)) = next_position(*i as isize, *j as isize, di, dj, grid) {
        if grid[ni][nj] == b'#' {
            *direction = (*direction + 1) % 4;
            return true;
        }
        grid[ni][nj] = b'X';
        *i = ni;
        *j = nj;
    }
    false
}

fn next_position(
    i: isize,
    j: isize,
    di: isize,
    dj: isize,
    grid: &[Vec<u8>],
) -> Option<(usize, usize)> {
    let ni = i + di;
    let nj = j + dj;
    if ni >= 0 && nj >= 0 && (ni as usize) < grid.len() && (nj as usize) < grid[0].len() {
        Some((ni as usize, nj as usize))
    } else {
        None
    }
}

fn count_visited(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .flatten()
        .filter(|&&ch| ch == b'X')
        .count()
}


