mod ginfin;
use crate::ginfin::engine::{Grid, draw};

fn main() {
    let mut grid = Grid::new();

    grid.set_vline(25, 5, 40);
    grid.set_rectangle(10, 10, 30, 30);
    grid.set_hline(5, 25, 40);
    grid.set_text(11, 11, "This is a box".to_string());
    grid.set_hline(0, 5, 1000);
    draw(grid);
}
