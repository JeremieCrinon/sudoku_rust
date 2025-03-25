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
        [0, 7, 0, 0, 3, 0, 2, 0, 0],
        [0, 0, 5, 0, 0, 2, 9, 0, 0],
        [4, 0, 0, 9, 0, 0, 0, 0, 0],
        [0, 0, 4, 2, 0, 5, 0, 9, 0],
        [0, 1, 0, 3, 9, 0, 7, 0, 6],
        [2, 0, 0, 0, 0, 0, 0, 0, 5],
        [1, 9, 2, 7, 0, 0, 0, 3, 0],
        [0, 4, 7, 5, 0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0, 3, 0, 0, 0],
    ];

    for row in 0..9 {
        for col in 0..9 {
            let value = preset_grid[row][col];
            grid.set(row, col, value);
        }
    }

    grid
}