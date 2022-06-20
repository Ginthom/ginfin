pub trait Line {
    pub fn horizontal(&self)  -> char;
    pub fn vertical(&self)    -> char;
    pub fn upper_left(&self)  -> char;
    pub fn upper_right(&self) -> char;
    pub fn lower_left(&self)  -> char;
    pub fn lower_right(&self) -> char;
}

pub struct DoubleLine;
pub struct SingleLine;
pub struct ThickLine;


impl Line for DoubleLine {
    pub fn horizontal(&self)  -> char { '═' }
    pub fn vertical(&self)    -> char { '║' }
    pub fn upper_left(&self)  -> char { '╔' }
    pub fn upper_right(&self) -> char { '╗' }
    pub fn lower_left(&self)  -> char { '╚' }
    pub fn lower_right(&self) -> char { '╝' }
}

impl Line for SingleLine {
    pub fn horizontal(&self)  -> char { '─' }
    pub fn vertical(&self)    -> char { '│' }
    pub fn upper_left(&self)  -> char { '┌' }
    pub fn upper_right(&self) -> char { '┐' }
    pub fn lower_left(&self)  -> char { '└' }
    pub fn lower_right(&self) -> char { '┘' }
}

impl Line for ThickLine {
    pub fn horizontal(&self)  -> char { '━' }
    pub fn vertical(&self)    -> char { '┃' }
    pub fn upper_left(&self)  -> char { '┏' }
    pub fn upper_right(&self) -> char { '┓' }
    pub fn lower_left(&self)  -> char { '┗' }
    pub fn lower_right(&self) -> char { '┛' }
}
