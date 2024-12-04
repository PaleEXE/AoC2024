pub fn run(input: String) {
    let mut count = 0;

    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows-2{
        for col in 0..cols-2 {
            if check_x_mas(&grid, row, col) {
                count += 1;
            }
        }
    }
    println!("{}", count)
}

fn check_x_mas(grid: &[&[u8]], start_row: usize, start_col: usize) -> bool {

    let x0y0 = grid[start_row][start_col];
    let x0y2 = grid[start_row][start_col + 2];
    let x1y1 = grid[start_row + 1][start_col + 1];
    let x2y0 = grid[start_row + 2][start_col];
    let x2y2 = grid[start_row + 2][start_col + 2];

    (x0y0 == x2y0 || x0y0 == x0y2) && (x0y0 == b'M' || x0y0 == b'S') &&
    (x2y2 == x2y0 || x2y2 == x0y2) && (x2y2 == b'M' || x2y2 == b'S') &&
    (x0y0 != x2y2) && x1y1 == b'A'
}