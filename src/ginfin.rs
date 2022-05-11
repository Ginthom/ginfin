pub mod engine {
    use terminal_size::{Width, Height, terminal_size};
    use std::process::Command;

    struct DoubleLine{}
    impl DoubleLine{
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

    struct ThinLine{}
    impl ThinLine{
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

    struct FatLine{}
    impl FatLine{
        const HOR: char = '━';
        const VER: char = '┃';
        const CRS: char = '╋'; 
        const UDR: char = '┣';
        const UDL: char = '┫';
        const DLR: char = '┳';
        const ULR: char = '┻';
        const CDR: char = '┏';
        const CDL: char = '┓';
        const CUR: char = '┗';
        const CUL: char = '┛';
    }

    struct Line<T> {
        style: T
    }

    impl<T> Line<T> {
        pub fn new(style: T) -> Line<T> {
            return Line::<T> {
                style: style
            };
        }

        pub fn set_style(&mut self, style: T) {
            self.style = style;    
        }
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
                    DoubleLine::VER => self.set_pixel(x+i, y, DoubleLine::CRS),
                    _            => self.set_pixel(x+i, y, DoubleLine::HOR)
                }
            }
        }

        pub fn set_vline(&mut self, x: usize, y: usize, length: usize) {
            for i in 0..length {
                match self.get_pixel(x, y+i) {
                    DoubleLine::HOR => self.set_pixel(x, y+i, DoubleLine::CRS),
                    _            => self.set_pixel(x, y+i, DoubleLine::VER)
                }
            }
        }

        pub fn set_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize) {
            self.set_pixel(x, y, DoubleLine::CDR);
            self.set_pixel(x+width, y, DoubleLine::CDL);
            self.set_pixel(x, y+height, DoubleLine::CUR);
            self.set_pixel(x+width, y+height, DoubleLine::CUL);
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
