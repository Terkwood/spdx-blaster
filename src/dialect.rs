use std::path::Path;

use crate::comment::*;

#[derive(Debug, Clone, Copy)]
pub enum Dialect {
    Rust,
    Shell,
    Unknown,
}

impl Dialect {
    pub fn from(target: &Path) -> Dialect {
        if let Some(os_file_name) = target.file_name() {
            let file_name = os_file_name.to_string_lossy();
            let s: Vec<&str> = file_name.split('.').collect();
            if s.is_empty() {
                Dialect::Unknown
            } else {
                s.last()
                    .and_then(|ext| {
                        Some(match ext.to_ascii_lowercase().trim() {
                            "rs" => Dialect::Rust,
                            "sh" => Dialect::Shell,
                            _ => Dialect::Unknown,
                        })
                    })
                    .unwrap_or(Dialect::Unknown)
            }
        } else {
            Dialect::Unknown
        }
    }

    pub fn comment(self) -> Comment {
        match self {
            Dialect::Rust => Comment::DoubleFwdSlash,
            Dialect::Shell => Comment::Pound,
            Dialect::Unknown => Comment::Empty,
        }
    }
}
