mod ginfin;
use crate::ginfin::engine::Grid;

struct grid {
    pub width: u32,
    pub height: u32,
    pub grid: [[char; width]; height]
}

fn main() {
    let grid = Grid::new();

    println!("Terminal Size: {}x | {}y.", grid.bounds.width, grid.bounds.height);
    
}
