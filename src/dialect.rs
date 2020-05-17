use crate::comment::*;
#[derive(Debug, Clone, Copy)]
pub enum Dialect {
    Rust,
    Shell,
}

impl Dialect {
    fn from_file(file_name: &str) -> Option<Dialect> {
        let s: Vec<&str> = file_name.split('.').collect();
        if s.is_empty() {
            None
        } else {
            s.last()
                .and_then(|ext| match ext.to_ascii_lowercase().trim() {
                    "rs" => Some(Dialect::Rust),
                    "sh" => Some(Dialect::Shell),
                    _ => None,
                })
        }
    }

    fn comment(self) -> Comment {
        match self {
            Dialect::Rust => Comment::DoubleFwdSlash,
            Dialect::Shell => Comment::Pound,
        }
    }
}
