// SPDX-License-Identifier: MIT
use std::fmt;

use crate::ids::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum License {
    Apache20,
    MIT,
    GPL20Only,
    GPL30OrLater,
}

const PREFIX: &str = "SPDX-License-Identifier: ";

impl Default for License {
    fn default() -> Self {
        License::MIT
    }
}

impl License {
    pub fn from(s: &str) -> Self {
        match s.trim().to_lowercase() {
            t if License::Apache20.compare_lc(&t) => License::Apache20,
            t if License::GPL20Only.compare_lc(&t) => License::GPL20Only,
            t if License::GPL30OrLater.compare_lc(&t) => License::GPL30OrLater,
            t if License::MIT.compare_lc(&t) => License::MIT,
            _ => License::default(),
        }
    }

    pub fn id(self) -> String {
        match self {
            License::Apache20 => ID_APACHE20,
            License::GPL20Only => ID_GPL_20_ONLY,
            License::GPL30OrLater => ID_GPL_30_OR_LATER,
            License::MIT => ID_MIT,
        }
        .to_string()
    }

    fn compare_lc(self, t: &str) -> bool {
        t == self.id().trim().to_lowercase()
    }
}

/// This implementation of Display must conform exactly
/// to SPDX license strings.
///
/// It uses https://spdx.org/licenses/ as the source of
/// truth for these strings.
impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", PREFIX, self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(License::MIT.to_string(), "SPDX-License-Identifier: MIT");
    }

    #[test]
    fn from() {
        assert_eq!(License::from("MIT"), License::MIT);
        assert_eq!(License::from("GPL-3.0-or-later"), License::GPL30OrLater);
    }
}
