use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Comment {
    DoubleFwdSlash,
    Pound,
    Empty,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Comment::DoubleFwdSlash => "// ",
            Comment::Pound => "# ",
            Comment::Empty => "",
        };
        write!(f, "{}", s)
    }
}
