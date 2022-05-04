pub mod engine {
    use terminal_size::{Width, Height, terminal_size};
    use std::process::Command;

    struct Lines{}
    impl Lines {
        const D_HOR: char = '\u{2550}';
        const D_VER: char = '\u{2551}';
        const D_CRS: char = '\u{256c}';
    }

    pub struct Dimension {
        pub width:  usize,
        pub height: usize
    }

    impl Dimension {
        fn new() -> Dimension {
            return match get_dimensions() {
                Ok(dim) => dim,
                Err(msg) => panic!("{}", msg)
            };
        }
    }

    pub struct Row {
        width:   usize,
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
        pub rows: Vec<Row>
    }

    impl Grid {
        pub fn new() -> Grid {
            let mut grid: Grid = Grid {
                bounds: Dimension::new(),
                rows: Vec::<Row>::new()
            };

            for _ in 1..grid.bounds.height {
                grid.rows.push(Row::new(grid.bounds.width));
            }

            return grid;
        }

        pub fn set_pixel(&mut self, x: usize, y: usize, content: char) {
            self.check_pos(x, y);
            self.rows[y].pixel[x] = content;
        }

        pub fn set_text(&mut self, x: usize, y: usize, content: String) {
            for i in 0..content.len() {
                self.set_pixel(x+i, y, content.chars()
                                              .nth(i)
                                              .unwrap());
            }
        }

        pub fn set_hline(&mut self, x: usize, y: usize, length: usize) {
            for i in 0..length {
                match self.rows[y].pixel[x+i] {
                    Lines::D_VER => self.set_pixel(x+i, y, Lines::D_CRS),
                    _            => self.set_pixel(x+i, y, Lines::D_HOR)
                }
            }
        }

        pub fn set_vline(&mut self, x: usize, y: usize, length: usize) {
            for i in 0..length {
                match self.rows[y+i].pixel[x] {
                    Lines::D_HOR => self.set_pixel(x, y+i, Lines::D_CRS),
                    _            => self.set_pixel(x, y+i, Lines::D_VER)
                }
            }
        }

        fn check_pos(&self, x: usize, y: usize) {
            if x > self.bounds.width
            || y > self.bounds.height {
                panic!("Pixel out of bounds\nSet: {}/{} Max: {}/{}\n",
                       x, y,
                       self.bounds.width, self.bounds.height)
            }
        }
    }

    pub fn draw(grid: Grid) {
        Command::new("clear")
                 .status()
                 .unwrap();
        for row in grid.rows {
            for pixel in row.pixel {
                print!("{}", pixel);
            }
            print!("\n");
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
}
