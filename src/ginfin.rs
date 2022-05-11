// Copyright 2022 Thomas Gingele (https://github.com/Ginthom)

pub mod engine {
    pub mod style;

    use terminal_size::{Width, Height, terminal_size};
    use std::process::Command;
    use crate::ginfin::engine::style::Line;

    pub struct Dimension {
        pub width:  usize,
        pub height: usize
    }

    impl Dimension {
        fn new() -> Dimension {
            return match get_dimensions() {
                Ok(dim)  => dim,
                Err(msg) => panic!("{}", msg)
            };
        }
    }

    pub struct Row {
        width: usize,
        pixel: Vec<char>
    }

    impl Row {
        fn new(length: usize) -> Row {
            let mut row: Row = Row {
                width: length,
                pixel: Vec::<char>::new()
            };

            for _ in 1..row.width {
                row.pixel.push(' ');
            }

            return row;
        }
    }

    pub struct Grid {
        pub bounds: Dimension,
        pub rows:   Vec<Row>
    }

    impl Grid {
        pub fn new() -> Grid {
            let mut grid: Grid = Grid {
                bounds: Dimension::new(),
                rows:   Vec::<Row>::new()
            };

            for _ in 1..grid.bounds.height {
                grid.rows.push(Row::new(grid.bounds.width));
            }

            return grid;
        }

        fn check_pos(&self, x: usize, y: usize) -> Result<bool, String> {
            match x < self.bounds.width-1
               && y < self.bounds.height-1 {
                true  => Ok(true),
                false => Err("Pixel out of bounds".to_string())
            }
        }

        // ELEMENT SETTER

        pub fn set_pixel(&mut self, x: usize, y: usize, content: char) {
            match self.check_pos(x, y) {
                Ok(_)  => self.rows[y].pixel[x] = content,
                Err(_) => return
            };
        }

        pub fn set_text(&mut self, x: usize, y: usize, content: String) {
            for i in 0..content.len() {
                self.set_pixel(x+i, y, content.chars()
                                              .nth(i)
                                              .unwrap());
            }
        }

        pub fn set_hline(&mut self, x: usize, y: usize, length: usize, line: &dyn Line) {
            for i in 0..length {
                if self.get_pixel(x+i, y) == line.VER() {
                    self.set_pixel(x+i, y, line.CRS());
                } else {
                    self.set_pixel(x+i, y, line.HOR());
                }
            }
        }

        pub fn set_vline(&mut self, x: usize, y: usize, length: usize, line: &dyn Line) {
            for i in 0..length {
                if self.get_pixel(x, y+i) == line.HOR() {
                    self.set_pixel(x, y+i, line.CRS());
                } else {
                    self.set_pixel(x, y+1, line.VER());
                }
            }
        }

        pub fn set_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, line: &dyn Line) {
            self.set_pixel(x,       y,        line.CDR());
            self.set_pixel(x+width, y,        line.CDL());
            self.set_pixel(x,       y+height, line.CUR());
            self.set_pixel(x+width, y+height, line.CUL());

            self.set_hline(x+1,     y,        width-1,  line);
            self.set_hline(x+1,     y+height, width-1,  line);
            self.set_vline(x,       y+1,      height-1, line);
            self.set_vline(x+width, y+1,      height-1, line);
        }

        // ELEMENT GETTER

        pub fn get_pixel(&self, x: usize, y: usize) -> char {
            match self.check_pos(x, y) {
                Ok(_)  => self.rows[y].pixel[x],
                Err(_) => ' '
            }
        }
    }

    fn get_dimensions() -> Result<Dimension, String> {
        let term_size = terminal_size();
        let mut dim   = Dimension{width: 0, height: 0};

        if let Some((Width(x), Height(y))) = term_size {
            dim.width  = x as usize;
            dim.height = y as usize;
        }

        match dim.width  > 0
           && dim.height > 0 {
            true  => Ok(dim),
            false => Err("Failed to get terminal size!".to_string())
        }
    }

    pub fn draw(grid: Grid) {
        Command::new("clear")
                .status()
                .expect("Failed to clear terminal!");
        for row in grid.rows {
            for pixel in row.pixel {
                print!("{}", pixel);
            }
            print!("\n");
        }
    }
}
