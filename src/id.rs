use std::fmt;

pub enum Id {
    MIT,
}

const PREFIX: &str = "SPDX-License-Identifier: ";

/// See https://spdx.dev/ids/
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Id::MIT => "MIT",
        };
        write!(f, "{}{}", PREFIX, s)
    }
}
