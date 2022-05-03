pub mod engine { 
    use terminal_size::{Width, Height, terminal_size};

    pub struct Dimension {
        pub width:  u16,
        pub height: u16
    }

    impl Dimension {
        pub fn new() -> Dimension {
            return match get_dimensions() {
                Ok(dim) => dim,
                Err(msg) => panic!("{}", msg)
            };
        }
    }

    pub struct Row {
        width:   u16,
        char_at: Vec<char>
    }

    impl Row {
        pub fn new(length: u16) -> Row {
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
        row_at: Vec<Row>
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
    }

    pub fn get_dimensions() -> Result<Dimension, String> {
        let term_size = terminal_size();
        let mut dim   = Dimension{width: 0, height: 0};

        if let Some((Width(x), Height(y))) = term_size {
            dim.width  = x;
            dim.height = y;
        }

        match dim.width  > 0
           && dim.height > 0 {
            true  => Ok(dim),
            false => Err("Failed to get terminal size!".to_string())
        }
    }
}
