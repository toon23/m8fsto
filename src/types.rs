use std::{fmt::Display, path::PathBuf};


#[derive(Debug, Clone)]
pub enum M8FstoErr {
    UnparseableM8File { path: PathBuf, reason: String },
    InvalidSearchPattern { pattern: String },
    CannotReadFile { path: PathBuf, reason: String },
    MultiErrs { inner: Vec<M8FstoErr> },
    InvalidPath
}

impl Display for M8FstoErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            M8FstoErr::UnparseableM8File { path, reason } => {
                writeln!(f, "Can't parse M8 file '{:?}' : {}", path.as_path(), reason)
            }
            M8FstoErr::InvalidSearchPattern { pattern } => {
                writeln!(f, "Invalid search pattern '{}'", pattern)
            },
            M8FstoErr::CannotReadFile { path, reason } => {
                writeln!(f, "Cannot read file '{:?}' : {}", path, reason)
            },
            M8FstoErr::MultiErrs { inner } => {
                for i in inner.iter() {
                    i.fmt(f)?
                }
                Ok(())
            }
            M8FstoErr::InvalidPath => {
                writeln!(f, "Invalid path")
            }
        }
    }
}