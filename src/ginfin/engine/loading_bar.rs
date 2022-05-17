trait Style {
    fn front() -> char;
    fn end()   -> char;
    fn empty() -> char;
    fn full()  -> char;
}

struct Diamond {}
impl Style for Diamond {
    fn front() -> char { return '<'; }
    fn end()   -> char { return '>'; }
    fn empty() -> char { return '◇'; }
    fn full()  -> char { return '◆'; }
}

struct Round {}
impl Style for Round {
    fn front() -> char { return '('; }
    fn end()   -> char { return ')'; }
    fn empty() -> char { return '◯'; }
    fn full()  -> char { return '◉'; }

}

struct Cube {}
impl Style for Cube {
    fn front() -> char { return '['; }
    fn end()   -> char { return ']'; }
    fn empty() -> char { return '◻'; }
    fn full()  -> char { return '◼'; }

}

struct Full {}
impl Style for Full {
    fn front() -> char { return '['; }
    fn end()   -> char { return ']'; }
    fn empty() -> char { return ' '; }
    fn full()  -> char { return '▆'; }

}
