pub mod engine {
    use terminal_size::{Width, Height, terminal_size};
    use std::process::Command;

    struct Lines{}
    impl Lines {
        const D_HOR: char = '\u{2550}'; // ═
        const D_VER: char = '\u{2551}'; //  ║
        const D_CRS: char = '\u{256c}'; // ╬
        const D_UDR: char = '\u{2560}'; //  ╠
        const D_UDL: char = '\u{2563}'; // ╣
        const D_DLR: char = '\u{2566}'; //  ╦
        const D_ULR: char = '\u{2569}'; // ╩
        const D_CDR: char = '\u{2554}'; //  ╔
        const D_CDL: char = '\u{2557}'; // ╗
        const D_CUR: char = '\u{255a}'; //  ╚
        const D_CUL: char = '\u{255d}'; // ╝
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
                    Lines::D_VER => self.set_pixel(x+i, y, self.get_edge_pixel(x, y)),
                    _            => self.set_pixel(x+i, y, Lines::D_HOR)
                }
            }
        }

        pub fn set_vline(&mut self, x: usize, y: usize, length: usize) {
            for i in 0..length {
                match self.get_pixel(x, y+i) {
                    Lines::D_HOR => self.set_pixel(x, y+i, self.get_edge_pixel(x, y)),
                    _            => self.set_pixel(x, y+i, Lines::D_VER)
                }
            }
        }

        pub fn set_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize) {
            // TODO implement edge pixel for rectangle corners
            self.set_pixel(x, y, Lines::D_CDR);
            self.set_pixel(x+width, y, Lines::D_CDL);
            self.set_pixel(x, y+height, Lines::D_CUR);
            self.set_pixel(x+width, y+height, Lines::D_CUL);
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

        fn get_edge_pixel(&self, x: usize, y: usize) -> char {
            match (self.get_pixel(x-1, y),
                   self.get_pixel(x+1, y),
                   self.get_pixel(x, y-1),
                   self.get_pixel(x, y+1)) {
                (Lines::D_HOR, Lines::D_HOR, Lines::D_VER, Lines::D_VER) => Lines::D_CRS,
                (Lines::D_HOR, Lines::D_HOR, Lines::D_VER, ' ')          => Lines::D_ULR,
                (Lines::D_HOR, Lines::D_HOR, ' ',          Lines::D_HOR) => Lines::D_DLR,
                (Lines::D_HOR, Lines::D_HOR, ' ',          ' ')          => Lines::D_HOR, //Horizontal line section
                (Lines::D_HOR, ' ',          Lines::D_VER, Lines::D_VER) => Lines::D_UDL,
                (Lines::D_HOR, ' ',          Lines::D_VER, ' ')          => Lines::D_CUL,
                (Lines::D_HOR, ' ',          ' ',          Lines::D_VER) => Lines::D_CDL,
                (Lines::D_HOR, ' ',          ' ',          ' ')          => Lines::D_HOR, //Line end
                (' ',          Lines::D_HOR, Lines::D_VER, Lines::D_VER) => Lines::D_UDR,
                (' ',          Lines::D_HOR, Lines::D_VER, ' ')          => Lines::D_CUR,
                (' ',          Lines::D_HOR, ' ',          Lines::D_VER) => Lines::D_CDR,
                (' ',          Lines::D_HOR, ' ',          ' ')          => Lines::D_HOR, //Line end
                (' ',          ' ',          Lines::D_VER, Lines::D_VER) => Lines::D_VER, //Vertical line section
                (' ',          ' ',          Lines::D_VER, ' ')          => Lines::D_VER, //Line end
                (' ',          ' ',          ' ',          Lines::D_VER) => Lines::D_VER, //Line end
                                                                       _ => ' '
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
