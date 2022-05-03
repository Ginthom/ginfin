mod ginfin;
use crate::ginfin::engine::Grid;

fn main() {
    let mut grid = Grid::new();
    match grid.set_pixel(10, 10, 'A') {
        Ok((x, y)) => println!("Set pixel at {}x | {}y", x, y),
        Err(msg)   => println!("{}", msg)
    };

    println!("Terminal Size: {}x | {}y.", grid.bounds.width, grid.bounds.height);
    
}
