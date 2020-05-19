// SPDX-License-Identifier: MIT
use std::path::Path;

use crate::comment::*;

#[derive(Debug, Clone, Copy)]
pub enum Dialect {
    C,
    CPlusPlus,
    CSharp,
    Godot,
    Haskell,
    Java,
    Javascript,
    Python,
    Rust,
    Scala,
    Shell,
    Typescript,
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
                    .map(|ext| match ext.to_ascii_lowercase().trim() {
                        "cs" => Dialect::CSharp,
                        "rs" => Dialect::Rust,
                        "ts" => Dialect::Typescript,
                        "sc" => Dialect::Scala,
                        "scala" => Dialect::Scala,
                        "sh" => Dialect::Shell,
                        _ => Dialect::Unknown,
                    })
                    .unwrap_or(Dialect::Unknown)
            }
        } else {
            Dialect::Unknown
        }
    }

    pub fn comment(self) -> Comment {
        match self {
            Dialect::Java => Comment::DoubleFwdSlash,
            Dialect::Javascript => Comment::DoubleFwdSlash,
            Dialect::Rust => Comment::DoubleFwdSlash,
            Dialect::Scala => Comment::DoubleFwdSlash,
            Dialect::Shell => Comment::Pound,
            Dialect::Unknown => Comment::Empty,
            _ => Comment::Empty,
        }
    }
}
