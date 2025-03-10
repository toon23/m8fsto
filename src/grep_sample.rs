use std::fs;
use std::path::Path;
use glob::glob;
use glob::Pattern;
use m8_files::{reader::*, Instrument};

use crate::types::M8FstoErr;

fn on_file_blob(cwd: &Path, pattern: &Pattern, path: &Path, data: Vec<u8>) -> Result<(), M8FstoErr> {
    let mut reader = Reader::new(data);
    let song = m8_files::Song::read_from_reader(&mut reader)
        .map_err(|e| M8FstoErr::UnparseableM8File {
            path: path.to_path_buf(),
            reason: format!("{:?}", e)
        })?;

    for (i, instr) in song.instruments.iter().enumerate() {
        match instr {
            Instrument::Sampler(sampler) if pattern.matches(&sampler.sample_path) => {
                let rel_path =
                    path.strip_prefix(cwd).unwrap_or(path);

                println!("{}:{:02X} {} : {}", 
                    rel_path.display().to_string(),
                    i,
                    sampler.name,
                    sampler.sample_path);
            }
            _ => {}
        }
    }

    Ok(())
}

fn on_dir(cwd: &Path, pattern: &Pattern, path: &str) -> Result<(), M8FstoErr> {
    let mut errors = vec![];

    for entry in glob(path).expect("Failed to read glob pattern") {
        match entry {
            Err(e) => println!("{:?}", e),
            Ok(path) => {
                let try_as_file = fs::read(&path);
                match try_as_file {
                    Err(e) => {
                        errors.push(M8FstoErr::CannotReadFile {
                            path: path.to_path_buf(),
                            reason: format!("{:?}", e)
                        })
                    }
                    Ok(file_blob) => {
                        match on_file_blob(cwd, pattern,&path, file_blob) {
                            Ok(()) => {}
                            Err(m8err) => errors.push(m8err),
                        }
                    }
                }
            }
        }
    }

    if errors.len() == 0 {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors[0].clone())
    } else {
        Err(M8FstoErr::MultiErrs { inner: errors })
    }
}

/// Try to list sample of a given path
pub fn grep_sample(cwd: &Path, pattern: &str, path : &Option<String>) -> Result<(), M8FstoErr> {
    let pat =
        glob::Pattern::new(pattern)
            .map_err(|e|
                M8FstoErr::InvalidSearchPattern { pattern: format!("{:?}", e) })?;

    let mut errors = vec![];
    match path {
        None => {
            match on_dir(cwd, &pat, "./") {
                Ok(_) => {}
                Err(e) => errors.push(e)
            }
        },
        Some(path) => {
            let try_as_file = fs::read(path);
            match try_as_file {
                Err(_) => {
                    match on_dir(cwd, &pat, path) {
                        Ok(()) => {}
                        Err(e) => errors.push(e)
                    }
                }
                Ok(file_blob) => {
                    let as_path = Path::new(path);
                    match on_file_blob(cwd, &pat, as_path, file_blob) {
                        Ok(()) => {}
                        Err(e) => errors.push(e)
                    }
                }
            }
        }
    };

    if errors.len() == 0 {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors[0].clone())
    } else {
        Err(M8FstoErr::MultiErrs { inner: errors })
    }
}