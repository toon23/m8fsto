use std::{fs, path::Path};
use glob::glob;
use m8_files::{reader::*, Instrument};

use crate::types::M8FstoErr;

fn on_file_blob(cwd: &Path, path: &Path, data: Vec<u8>) -> Result<(), M8FstoErr> {
    let mut reader = Reader::new(data);
    let song = m8_files::Song::read_from_reader(&mut reader)
        .map_err(|e| M8FstoErr::UnparseableM8File {
            path: path.to_path_buf(),
            reason: format!("{:?}", e)
        })?;

    let mut has_seen_sample = false;
    for (i, instr) in song.instruments.iter().enumerate() {
        match instr {
            Instrument::Sampler(sampler) => {
                if !has_seen_sample {
                    let rel_path =
                        path.strip_prefix(cwd).unwrap_or(path);

                    println!("\n{}", rel_path.display().to_string());
                    has_seen_sample = true;
                }
                if sampler.name.len() > 0 {
                    println!("  {:02X} {} : {}", i, sampler.name, sampler.sample_path);
                } else {
                    println!("  {:02X} : {}", i, sampler.sample_path);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn on_dir(cwd: &Path, path: &str) -> Result<(), M8FstoErr> {
    let mut errors = vec![];

    for entry in glob(path)
        .map_err(|e|M8FstoErr::InvalidSearchPattern { pattern: format!("{:?}", e) })? {
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
                        match on_file_blob(cwd, path.as_path(), file_blob) {
                            Ok(()) => {},
                            Err(e) => errors.push(e)
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
pub fn ls_sample(cwd: &Path, path : &Option<String>) -> Result<(), M8FstoErr> {
    match path {
        None => on_dir(cwd, "./"),
        Some(path) => {
            let try_as_file = fs::read(path);
            match try_as_file {
                Err(_) => { on_dir(cwd, path) }
                Ok(file_blob) => {
                    let as_path = Path::new(path);
                    on_file_blob(cwd, as_path, file_blob)
                }
            }
        }
    }
}