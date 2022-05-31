use terminal_size::{Width, Height, terminal_size};

pub struct Grid {
    width:  usize,
    height: usize,
    pixel: Vec<Vec<char>>
}

impl Grid {
    fn init_dimensions(&mut self) {
        if let Some((Width(x), Height(y))) = terminal_size() {
            self.width  = x as usize;
            self.height = y as usize;
        }
    }

    fn init_grid(&mut self) {
        self.pixel = Vec::<Vec<char>>::new();

        for y in 1..self.height {
            self.pixel.push(Vec::<char>::new());

            for _ in 1..self.width {
                self.pixel[y].push(' ');
            } 
        }
    }

    pub fn new(&mut self) {
        self.init_dimensions();
        self.init_grid();
    }
}
