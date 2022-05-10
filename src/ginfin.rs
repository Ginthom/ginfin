pub mod engine {
    use terminal_size::{Width, Height, terminal_size};
    use std::process::Command;

    struct DOUBLE_LINES{}
    impl DOUBLE_LINES{
        const HOR: char = '═';
        const VER: char = '║';
        const CRS: char = '╬'; 
        const UDR: char = '╠';
        const UDL: char = '╣';
        const DLR: char = '╦';
        const ULR: char = '╩';
        const CDR: char = '╔';
        const CDL: char = '╗';
        const CUR: char = '╚';
        const CUL: char = '╝';
    }

    struct THIN_LINES{}
    impl THIN_LINES{
        const HOR: char = '─';
        const VER: char = '│';
        const CRS: char = '┼'; 
        const UDR: char = '├';
        const UDL: char = '┤';
        const DLR: char = '┬';
        const ULR: char = '┴';
        const CDR: char = '┌';
        const CDL: char = '┐';
        const CUR: char = '└';
        const CUL: char = '┘';
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
            match self.check_pos(x, y) {
                Ok(_) => self.rows[y].pixel[x] = content,
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

        pub fn set_hline(&mut self, x: usize, y: usize, length: usize) {
            for i in 0..length {
                match self.get_pixel(x+i, y) {
                    DOUBLE_LINES::VER => self.set_pixel(x+i, y, DOUBLE_LINES::CRS),
                    _            => self.set_pixel(x+i, y, DOUBLE_LINES::HOR)
                }
            }
        }

        pub fn set_vline(&mut self, x: usize, y: usize, length: usize) {
            for i in 0..length {
                match self.get_pixel(x, y+i) {
                    DOUBLE_LINES::HOR => self.set_pixel(x, y+i, DOUBLE_LINES::CRS),
                    _            => self.set_pixel(x, y+i, DOUBLE_LINES::VER)
                }
            }
        }

        pub fn set_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize) {
            self.set_pixel(x, y, DOUBLE_LINES::CDR);
            self.set_pixel(x+width, y, DOUBLE_LINES::CDL);
            self.set_pixel(x, y+height, DOUBLE_LINES::CUR);
            self.set_pixel(x+width, y+height, DOUBLE_LINES::CUL);
            self.set_hline(x+1, y, width-1);
            self.set_hline(x+1, y+height, width-1);
            self.set_vline(x, y+1, height-1);
            self.set_vline(x+width, y+1, height-1);
        }

        pub fn get_pixel(&self, x: usize, y: usize) -> char {
            match self.check_pos(x, y) {
                Ok(_)  => self.rows[y].pixel[x],
                Err(_) => ' '
            }
        }

        fn check_pos(&self, x: usize, y: usize) -> Result<bool, String> {
            match x < self.bounds.width-1
               && y < self.bounds.height-1 {
                true => Ok(true),
                false => Err("Pixel out of bounds".to_string())
            }
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
