use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect();

    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let mut anti_nodes: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch != b'.' {
                antennas.entry(ch).or_default().push((row, col));
            }
        }
    }

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (row1, col1) = positions[i];
                let (row2, col2) = positions[j];

                let delta_row = row2 as isize - row1 as isize;
                let delta_col = col2 as isize - col1 as isize;

                let mut anti_node_row1 = row1 as isize;
                let mut anti_node_col1 = col1 as isize;

                while is_in_bounds((anti_node_row1, anti_node_col1), &grid) {
                    anti_nodes.insert((anti_node_row1 as usize, anti_node_col1 as usize));

                    anti_node_row1 -= delta_row;
                    anti_node_col1 -= delta_col;
                }

                let mut anti_node_row2 = row2 as isize;
                let mut anti_node_col2 = col2 as isize;

                while is_in_bounds((anti_node_row2, anti_node_col2), &grid) {
                    anti_nodes.insert((anti_node_row2 as usize, anti_node_col2 as usize));

                    anti_node_row2 += delta_row;
                    anti_node_col2 += delta_col;
                }
            }
        }
    }

    println!("{}", anti_nodes.len());
}

fn is_in_bounds(position: (isize, isize), grid: &[Vec<u8>]) -> bool {
    position.0 >= 0
        && position.0 < grid.len() as isize
        && position.1 >= 0
        && position.1 < grid[0].len() as isize
}
