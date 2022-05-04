mod ginfin;
use crate::ginfin::engine::{Grid, draw};

fn main() {
    let mut grid = Grid::new();
    grid.set_hline(10, 10, 31);
    grid.set_hline(10, 30, 31);
    grid.set_vline(10, 10, 21);
    grid.set_vline(40, 10, 21);
    grid.set_text(11, 11, "This is a box".to_string());
    draw(grid);
}
