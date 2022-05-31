// Copyright 2022 Thomas Gingele (https://github.com/Ginthom)

pub mod engine {
    pub mod lines;
    pub mod loading_bars;

    use terminal_size::{Width, Height, terminal_size};
    use std::process::Command;
    use crate::ginfin::engine::lines::Line;
    use crate::ginfin::engine::loading_bars::{Bar, TinyBar};

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
                if self.get_pixel(x+i, y) == line.ver() {
                    self.set_pixel(x+i, y, line.crs());
                } else {
                    self.set_pixel(x+i, y, line.hor());
                }
            }
        }

        pub fn set_vline(&mut self, x: usize, y: usize, length: usize, line: &dyn Line) {
            for i in 0..length {
                if self.get_pixel(x, y+i) == line.hor() {
                    self.set_pixel(x, y+i, line.crs());
                } else {
                    self.set_pixel(x, y+i, line.ver());
                }
            }
        }

        pub fn set_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, line: &dyn Line) {
            self.set_pixel(x,       y,        line.cdr());
            self.set_pixel(x+width, y,        line.cdl());
            self.set_pixel(x,       y+height, line.cur());
            self.set_pixel(x+width, y+height, line.cul());

            self.set_hline(x+1,     y,        width-1,  line);
            self.set_hline(x+1,     y+height, width-1,  line);
            self.set_vline(x,       y+1,      height-1, line);
            self.set_vline(x+width, y+1,      height-1, line);
        }

        pub fn set_titlebox(&mut self, x: usize, y: usize, width: usize, height: usize, mut text: String, line: &dyn Line) {
            text.truncate(width-1);

            self.set_rectangle(x, y, width, height, line);
            self.set_text(x+1, y+1, text);
            self.set_hline(x, y+2, width, line);
            self.set_pixel(x, y+2, line.udr());
            self.set_pixel(x+width, y+2, line.udl());

        }

        pub fn set_loadingbar(&mut self, x: usize, y: usize, length: usize, progress: usize, bar: &dyn Bar) {
            self.set_pixel(x,        y, bar.front());
            self.set_pixel(x+length, y, bar.end());

            for i in 1..length {
                match i <= progress/length {
                    true  => self.set_pixel(x+i, y, bar.full()),
                    false => self.set_pixel(x+i, y, bar.empty())
                };
            }
        }

        pub fn set_tiny_loadingbar(&mut self, x: usize, y: usize, progress: usize) {
            self.set_pixel(x, y, TinyBar{}.get(progress));
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
