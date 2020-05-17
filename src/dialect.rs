use crate::comment::*;
#[derive(Debug, Clone, Copy)]
pub enum Dialect {
    Rust,
    Bash,
}

impl Dialect {
    fn comment(self) -> Comment {
        match self {
            Dialect::Rust => Comment::DoubleFwdSlash,
            Dialect::Bash => Comment::Pound,
        }
    }
}
