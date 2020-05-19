// SPDX-License-Identifier: MIT
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Comment {
    DoubleDash,
    DoubleFwdSlash,
    Hash,
    Empty,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Comment::DoubleDash => "-- ",
            Comment::DoubleFwdSlash => "// ",
            Comment::Hash => "# ",
            Comment::Empty => "",
        };
        write!(f, "{}", s)
    }
}
