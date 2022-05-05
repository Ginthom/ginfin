mod ginfin;
use crate::ginfin::engine::{Grid, draw};

fn main() {
    let mut grid = Grid::new();

    grid.set_rectangle(0, 3, grid.bounds.width-5, grid.bounds.height-1);
    draw(grid);
}
