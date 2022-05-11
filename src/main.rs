mod ginfin;

use crate::ginfin::engine::{Grid, draw};
use crate::ginfin::engine::style::Fat;

fn main() {
    let mut grid = Grid::new();

    grid.set_hline(5, 10, 10, &Fat{});
    draw(grid);
}
