mod ginfin;
use crate::ginfin::engine::{Grid, draw};

fn main() {
    let mut grid = Grid::new();
    grid.set_pixel(10, 10, 'A');
    grid.set_pixel(100, 27, 'X');
    draw(grid);
}
