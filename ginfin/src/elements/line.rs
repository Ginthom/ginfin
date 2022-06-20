// Copyright 2022 Thomas Gingele (https://github.com/b1tc0r3)

pub struct DoubleLine;
pub struct SingleLine;
pub struct ThickLine;

pub trait Line {
    fn horizontal(&self)  -> char;
    fn vertical(&self)    -> char;
    fn upper_left(&self)  -> char;
    fn upper_right(&self) -> char;
    fn lower_left(&self)  -> char;
    fn lower_right(&self) -> char;
}

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
}
