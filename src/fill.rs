use crate::grid;
use text_io::read;

pub fn fill(mut grid: grid::Grid) -> grid::Grid {
    for row in 0..9 {
        for col in 0..9 {
            grid.print();
            println!("Value for row {} col {}", row, col);
            let value: u8 = read!();
            grid.set(row, col, value);
        }
    }
    grid
}

pub fn fill_auto(mut grid: grid::Grid) -> grid::Grid {
    let preset_grid = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    for row in 0..9 {
        for col in 0..9 {
            let value = preset_grid[row][col];
            grid.set(row, col, value);
        }
    }

    grid
}