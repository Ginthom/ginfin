mod ginfin;

use crate::ginfin::engine::{Grid, draw};
use crate::ginfin::engine::style::{Fat, Thin, Double};

fn test_display() {
    let mut grid = Grid::new();

    grid.set_rectangle(0,  2, 10, 10, &Thin{});
    grid.set_rectangle(11, 2, 10, 10, &Fat{});
    grid.set_rectangle(22, 2, 10, 10, &Double{});

    grid.set_hline(1,  13, 9, &Thin{});
    grid.set_hline(12, 13, 9, &Fat{});
    grid.set_hline(23, 13, 9, &Double{});

    grid.set_vline(1,  13, 9, &Thin{});
    grid.set_vline(12, 13, 9, &Fat{});
    grid.set_vline(23, 13, 9, &Double{});

    grid.set_text(1, 17, "This is merely a test run".to_string());

    draw(grid);
}

fn main() {
    test_display();
}
