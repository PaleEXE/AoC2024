use std::collections::{HashSet};

pub fn run(input: String) {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|ch| *ch - 48).collect())
        .collect();

    let mut zeros: Vec<(usize, usize)> = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                zeros.push((i, j));
            }
        }
    }

    let mut rizz = 0;
    for zero in zeros {
        let mut nines: HashSet<(usize, usize)> = HashSet::new();
        dfs(&grid, zero.0, zero.1, &mut nines);
        rizz += nines.len()
    }
    println!("{}", rizz)
}

fn dfs(grid: &Vec<Vec<u8>>, i: usize, j: usize, nines: &mut HashSet<(usize, usize)>) {
    if grid[i][j] == 9 { 
        nines.insert((i, j));
    }

    if i != grid.len() - 1{
        if grid[i][j] + 1 == grid[i+1][j] {
            dfs(grid, i + 1, j, nines)
        }
    }
    if j != grid.len() - 1{
        if grid[i][j] + 1 == grid[i][j+1] {
            dfs(grid, i, j + 1, nines)
        }
    }
    if i != 0 {
        if grid[i][j] + 1 == grid[i-1][j] {
            dfs(grid, i - 1, j, nines)
        }
    }
    if j != 0 {
        if grid[i][j] + 1 == grid[i][j-1] {
            dfs(grid, i, j - 1, nines)
        }
    }
}
