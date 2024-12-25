use std::collections::HashSet;

pub fn run(input: String) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut rizz = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let plant_type = grid[r][c];
                let mut region_cells = HashSet::new();
                let mut stack = vec![(r as i32, c as i32)];

                while let Some((row, col)) = stack.pop() {
                    if row < 0 || row >= rows as i32 || col < 0 || col >= cols as i32 
                        || visited[row as usize][col as usize] 
                        || grid[row as usize][col as usize] != plant_type {
                        continue;
                    }

                    visited[row as usize][col as usize] = true;
                    region_cells.insert((row as usize, col as usize));

                    stack.push((row + 1, col));
                    stack.push((row - 1, col));
                    stack.push((row, col + 1));
                    stack.push((row, col - 1));
                }

                if !region_cells.is_empty() {
                    let area = region_cells.len() as u32;
                    let mut perimeter = 0;
                    for (row, col) in region_cells.iter() {
                        let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                        for (dr, dc) in neighbors {
                            let nr = *row as i32 + dr;
                            let nc = *col as i32 + dc;
                            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 
                                || !region_cells.contains(&(nr as usize, nc as usize)) {
                                perimeter += 1;
                            }
                        }
                    }
                    let price = area * perimeter;
                    rizz += price;
                }
            }
        }
    }

    println!("{}", rizz)
}