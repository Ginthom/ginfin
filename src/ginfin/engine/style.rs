pub trait Line {
    fn HOR(&self) -> char;
    fn VER(&self) -> char;
    fn CRS(&self) -> char;
    fn UDR(&self) -> char;
    fn UDL(&self) -> char;
    fn DLR(&self) -> char;
    fn ULR(&self) -> char;
    fn CDR(&self) -> char;
    fn CDL(&self) -> char;
    fn CUR(&self) -> char;
    fn CUL(&self) -> char;
}

pub struct Double{}
impl Line for Double {
    fn HOR(&self) -> char { '═' }
    fn VER(&self) -> char { '║' }
    fn CRS(&self) -> char { '╬' }
    fn UDR(&self) -> char { '╠' }
    fn UDL(&self) -> char { '╣' }
    fn DLR(&self) -> char { '╦' }
    fn ULR(&self) -> char { '╩' }
    fn CDR(&self) -> char { '╔' }
    fn CDL(&self) -> char { '╗' }
    fn CUR(&self) -> char { '╚' }
    fn CUL(&self) -> char { '╝' }
}

pub struct Thin{}
impl Line for Thin {
    fn HOR(&self) -> char { '─' }
    fn VER(&self) -> char { '│' }
    fn CRS(&self) -> char { '┼' }
    fn UDR(&self) -> char { '├' }
    fn UDL(&self) -> char { '┤' }
    fn DLR(&self) -> char { '┬' }
    fn ULR(&self) -> char { '┴' }
    fn CDR(&self) -> char { '┌' }
    fn CDL(&self) -> char { '┐' }
    fn CUR(&self) -> char { '└' }
    fn CUL(&self) -> char { '┘' }
}

pub struct Fat{}
impl Line for Fat {
    fn HOR(&self) -> char { '━' }
    fn VER(&self) -> char { '┃' }
    fn CRS(&self) -> char { '╋' }
    fn UDR(&self) -> char { '┣' }
    fn UDL(&self) -> char { '┫' }
    fn DLR(&self) -> char { '┳' }
    fn ULR(&self) -> char { '┻' }
    fn CDR(&self) -> char { '┏' }
    fn CDL(&self) -> char { '┓' }
    fn CUR(&self) -> char { '┗' }
    fn CUL(&self) -> char { '┛' }
}
