use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum License {
    MIT,
}

const PREFIX: &str = "SPDX-License-Identifier: ";

impl License {
    pub fn from(_s: &str) -> Self {
        License::MIT
    }
}

/// See https://spdx.dev/ids/
impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            License::MIT => "MIT",
        };
        write!(f, "{}{}", PREFIX, s)
    }
}
