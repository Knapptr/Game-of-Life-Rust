mod grid;
mod screen;
use grid::Grid;

use screen::{Render, Screen};

fn main() {
    let mut grid = Grid::new(50, 0.3);
    let screen = Screen::new(4);
    let mut count = 0;
    screen.do_loop(|| {
        println!("{}", grid);
        grid.generate_mut();
        count += 1;
        count == 50
    })
}
