use std::collections::HashSet;

pub fn run(input: String) {
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect();

    let n = grid.len();
    let m = grid[0].len();

    let mut ii = 0;
    let mut jj = 0;
    let mut found = false;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == b'^' {
                ii = i;
                jj = j;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    let dd = vec![-1, 0, 1, 0];

    let mut i = ii;
    let mut j = jj;
    let mut dir = 0;
    let mut og_seen: HashSet<(usize, usize)> = HashSet::new();

    loop {
        og_seen.insert((i, j));
        let next_i = (i as isize + dd[dir] as isize) as usize;
        let next_j = (j as isize + dd[(dir + 1) % 4] as isize) as usize;

        if next_i >= n || next_j >= m {
            break;
        }

        if grid[next_i][next_j] == b'#' {
            dir = (dir + 1) % 4;
        } else {
            i = next_i;
            j = next_j;
        }
    }

    let will_loop = |oi: usize, oj: usize, grid: &mut Vec<Vec<u8>>, ii: usize, jj: usize| -> bool {
        if grid[oi][oj] == b'#' {
            return false;
        }

        grid[oi][oj] = b'#';
        let mut i = ii;
        let mut j = jj;
        let mut dir = 0;
        let mut seen: HashSet<(usize, usize, usize)> = HashSet::new();

        loop {
            if seen.contains(&(i, j, dir)) {
                grid[oi][oj] = b'.';
                return true;
            }
            seen.insert((i, j, dir));

            let next_i = (i as isize + dd[dir] as isize) as usize;
            let next_j = (j as isize + dd[(dir + 1) % 4] as isize) as usize;

            if next_i >= grid.len() || next_j >= grid[0].len() {
                grid[oi][oj] = b'.';
                return false;
            }

            if grid[next_i][next_j] == b'#' {
                dir = (dir + 1) % 4;
            } else {
                i = next_i;
                j = next_j;
            }
        }
    };

    let mut rizz = 0;
    for &(oi, oj) in &og_seen {
        if oi == ii && oj == jj {
            continue;
        }
        let loop_found = will_loop(oi, oj, &mut grid, ii, jj);
        rizz += loop_found as usize;
    }

    println!("{}", rizz);
}
