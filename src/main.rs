mod ginfin;
use crate::ginfin::engine::Grid;

fn main() {
    let grid = Grid::new();

    println!("Terminal Size: {}x | {}y.", grid.bounds.width, grid.bounds.height);
    
}
