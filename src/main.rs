mod ginfin;

use crate::ginfin::engine::{Grid, draw};
use crate::ginfin::engine::lines::{Fat, Thin, Double};

fn test_display() {
    let mut grid = Grid::new();

    grid.set_rectangle(0,  2, 10, 10, &Thin{});
    grid.set_rectangle(11, 2, 10, 10, &Fat{});
    grid.set_rectangle(22, 2, 10, 10, &Double{});

    grid.set_hline(1, 13, 33, &Thin{});
    grid.set_hline(1, 15, 33, &Fat{});
    grid.set_hline(2, 17, 33, &Double{});

    grid.set_vline(1,  13, 9, &Thin{});
    grid.set_vline(12, 13, 9, &Fat{});
    grid.set_vline(23, 13, 9, &Double{});

    grid.set_text(1, 19, "This is merely a test run".to_string());

    grid.set_titlebox(0, 22, 10, 10, "A Box".to_string(), &Thin{});
    grid.set_titlebox(11, 22, 10, 10, "sjhfgsjdgfsjhdfgsjhgsdjhfgsjkhgfasdfhalsg".to_string(), &Fat{});
    grid.set_titlebox(22, 22, 10, 10, " Hello C:".to_string(), &Double{});

    draw(grid);
}

fn main() {
    test_display();
}
