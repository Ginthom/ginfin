pub mod engine { 
    use terminal_size::{Width, Height, terminal_size};

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
        char_at: Vec<char>
    }

    impl Row {
        fn new(length: usize) -> Row {
            let mut row: Row = Row {
                width: length, 
                char_at: Vec::<char>::new()
            };

            for _ in 1..row.width {
                row.char_at.push(' ');
            }

            return row;
        }
    }

    pub struct Grid {
        pub bounds: Dimension,
        pub row_at: Vec<Row>
    }

    impl Grid {
        pub fn new() -> Grid {
            let mut grid: Grid = Grid {
                bounds: Dimension::new(), 
                row_at: Vec::<Row>::new()
            };

            for _ in 1..grid.bounds.height {
                grid.row_at.push(Row::new(grid.bounds.width)); 
            }

            return grid;
        }

        pub fn set_pixel(&mut self, x: usize, y: usize, content: char) -> Result<(usize, usize), String> {
            self.row_at[y].char_at[x] = content;
            match x <= self.bounds.width
               && y <= self.bounds.height {
                true  => Ok((x, y)),
                false => Err("Pixel out of bounds!".to_string())
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
}
