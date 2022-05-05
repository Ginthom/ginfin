mod ginfin;
use crate::ginfin::engine::{Grid, draw};

fn main() {
    let mut grid = Grid::new();

    grid.set_rectangle(0, 3, grid.bounds.width-5, grid.bounds.height-5);
    grid.set_hline(5, 10, 10);
    grid.set_vline(10, 5, 10);
    draw(grid);
}
