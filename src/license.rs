// SPDX-License-Identifier: MIT
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum License {
    Apache20,
    MIT,
    GPL20Only,
    GPL30OrLater,
}

const PREFIX: &str = "SPDX-License-Identifier: ";

impl License {
    pub fn from(_s: &str) -> Self {
        License::MIT
    }
}

/// This implementation of Display must conform exactly
/// to SPDX license strings.
///
/// It uses https://spdx.org/licenses/ as the source of
/// truth for these strings.
impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            License::Apache20 => "Apache-2.0",
            License::MIT => "MIT",
            License::GPL20Only => "GPL-2.0-only",
            License::GPL30OrLater => "GPL-3.0-or-later",
        };
        write!(f, "{}{}", PREFIX, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial() {
        assert_eq!(License::MIT.to_string(), "SPDX-License-Identifier: MIT");
    }
}
