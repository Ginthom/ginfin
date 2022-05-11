pub trait Line {
    fn hor(&self) -> char;
    fn ver(&self) -> char;
    fn crs(&self) -> char;
    fn udr(&self) -> char;
    fn udl(&self) -> char;
    fn dlr(&self) -> char;
    fn ulr(&self) -> char;
    fn cdr(&self) -> char;
    fn cdl(&self) -> char;
    fn cur(&self) -> char;
    fn cul(&self) -> char;
}

pub struct Double{}
impl Line for Double {
    fn hor(&self) -> char { '═' }
    fn ver(&self) -> char { '║' }
    fn crs(&self) -> char { '╬' }
    fn udr(&self) -> char { '╠' }
    fn udl(&self) -> char { '╣' }
    fn dlr(&self) -> char { '╦' }
    fn ulr(&self) -> char { '╩' }
    fn cdr(&self) -> char { '╔' }
    fn cdl(&self) -> char { '╗' }
    fn cur(&self) -> char { '╚' }
    fn cul(&self) -> char { '╝' }
}

pub struct Thin{}
impl Line for Thin {
    fn hor(&self) -> char { '─' }
    fn ver(&self) -> char { '│' }
    fn crs(&self) -> char { '┼' }
    fn udr(&self) -> char { '├' }
    fn udl(&self) -> char { '┤' }
    fn dlr(&self) -> char { '┬' }
    fn ulr(&self) -> char { '┴' }
    fn cdr(&self) -> char { '┌' }
    fn cdl(&self) -> char { '┐' }
    fn cur(&self) -> char { '└' }
    fn cul(&self) -> char { '┘' }
}

pub struct Fat{}
impl Line for Fat {
    fn hor(&self) -> char { '━' }
    fn ver(&self) -> char { '┃' }
    fn crs(&self) -> char { '╋' }
    fn udr(&self) -> char { '┣' }
    fn udl(&self) -> char { '┫' }
    fn dlr(&self) -> char { '┳' }
    fn ulr(&self) -> char { '┻' }
    fn cdr(&self) -> char { '┏' }
    fn cdl(&self) -> char { '┓' }
    fn cur(&self) -> char { '┗' }
    fn cul(&self) -> char { '┛' }
}
