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
        dfs(&grid, zero.0, zero.1, &mut rizz);
    }
    println!("{}", rizz)
}

fn dfs(grid: &Vec<Vec<u8>>, i: usize, j: usize, rizz: &mut u32) {
    if grid[i][j] == 9 { 
        *rizz += 1;
    }

    if i != grid.len() - 1{
        if grid[i][j] + 1 == grid[i+1][j] {
            dfs(grid, i + 1, j, rizz)
        }
    }
    if j != grid.len() - 1{
        if grid[i][j] + 1 == grid[i][j+1] {
            dfs(grid, i, j + 1, rizz)
        }
    }
    if i != 0 {
        if grid[i][j] + 1 == grid[i-1][j] {
            dfs(grid, i - 1, j, rizz)
        }
    }
    if j != 0 {
        if grid[i][j] + 1 == grid[i][j-1] {
            dfs(grid, i, j - 1, rizz)
        }
    }
}
