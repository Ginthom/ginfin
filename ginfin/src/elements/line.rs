// Copyright 2022 Thomas Gingele (https://github.com/b1tc0r3)

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub enum LineType {
    Double,
    Normal,
    Big,
}

struct Line {
    pox_x: i32,
    pox_y: i32,
    length: usize,
    line_type: LineType,
    orientation: Orientation,
}

impl Line {
    pub fn new(
        &mut self,
        pos_x: i32,
        pox_y: i32,
        length: usize,
        line_type: LineType,
        orientation: Orientation,
    ) {
        self.pox_x = pos_x;
        self.pox_y = pox_y;
        self.length = length;
        self.line_type = line_type;
        self.orientation = orientation;
    }

    /*
    pub fn get_pixel(&self) -> Vec<Vec<char>> {

    }
    */
    //TODO add setters and getters

    //fn horizontal(&self)  -> char;
    //fn vertical(&self)    -> char;
    //fn upper_left(&self)  -> char;
    //fn upper_right(&self) -> char;
    //fn lower_left(&self)  -> char;
    //fn lower_right(&self) -> char;
}
/*
impl Line for DoubleLine {
    fn horizontal(&self)  -> char { '═' }
    fn vertical(&self)    -> char { '║' }
    fn upper_left(&self)  -> char { '╔' }
    fn upper_right(&self) -> char { '╗' }
    fn lower_left(&self)  -> char { '╚' }
    fn lower_right(&self) -> char { '╝' }
}

impl Line for SingleLine {
    fn horizontal(&self)  -> char { '─' }
    fn vertical(&self)    -> char { '│' }
    fn upper_left(&self)  -> char { '┌' }
    fn upper_right(&self) -> char { '┐' }
    fn lower_left(&self)  -> char { '└' }
    fn lower_right(&self) -> char { '┘' }
}

impl Line for ThickLine {
    fn horizontal(&self)  -> char { '━' }
    fn vertical(&self)    -> char { '┃' }
    fn upper_left(&self)  -> char { '┏' }
    fn upper_right(&self) -> char { '┓' }
    fn lower_left(&self)  -> char { '┗' }
    fn lower_right(&self) -> char { '┛' }
}*/
