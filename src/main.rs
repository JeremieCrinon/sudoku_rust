mod grid;
mod fill;
mod solver;
use text_io::read;

fn main() {
    let mut grid = grid::Grid::new();

    let mut pass: bool = false;
    let mut value: u8 = 0;

    while pass == false {
        println!("Do you want to fill the grid yourself (0) or have it filled for you (1)?");
        value = read!();

        if value != 0 && value != 1 {
            println!("Please enter 0 or 1 !");
        } else {
            pass = true;
        }
    }

    if value == 0 {
        grid = fill::fill(grid);
    } else {
        grid = fill::fill_auto(grid);
    }

    println!("{}", solver::check_grid(&grid));
    
    grid.print();
}
