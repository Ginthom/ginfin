pub trait Bar {
    fn front(&self) -> char;
    fn end(&self)   -> char;
    fn empty(&self) -> char;
    fn full(&self)  -> char;
}

pub struct Diamond {}
impl Bar for Diamond {
    fn front(&self) -> char { return '<'; }
    fn end(&self)   -> char { return '>'; }
    fn empty(&self) -> char { return '◇'; }
    fn full(&self)  -> char { return '◆'; }
}

pub struct Round {}
impl Bar for Round {
    fn front(&self) -> char { return '('; }
    fn end(&self)   -> char { return ')'; }
    fn empty(&self) -> char { return '◯'; }
    fn full(&self)  -> char { return '◉'; }

}

pub struct Cube {}
impl Bar for Cube {
    fn front(&self) -> char { return '['; }
    fn end(&self)   -> char { return ']'; }
    fn empty(&self) -> char { return '◻'; }
    fn full(&self)  -> char { return '◼'; }

}

pub struct Full {}
impl Bar for Full {
    fn front(&self) -> char { return '['; }
    fn end(&self)   -> char { return ']'; }
    fn empty(&self) -> char { return ' '; }
    fn full(&self)  -> char { return '▆'; }

}

pub struct TinyBar {}
impl TinyBar {
    pub fn get(&self, progress: usize) -> char {
        let bars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        for i in 0..7 {
            match i*12 >= progress {
                true  => return bars[i],
                false => continue
            };
        }
        return bars[7];
    }
}
