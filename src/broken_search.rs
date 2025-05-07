use std::{collections::hash_map::Entry, path::PathBuf};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use m8_files::{reader::*, Instrument};

use crate::types::M8FstoErr;

pub(crate) fn is_sample_absolute(sample_path: &str) -> bool {
    let ch = sample_path.chars().nth(0).unwrap();
    ch == '/'
}

pub(crate) fn sample_to_absolute_path(
    backup_root: &Path,
    song_path: &Path,
    sample_path: &str) -> PathBuf {

    if is_sample_absolute(sample_path) {
        let rel_path : String = sample_path.chars().skip(1).collect();
        backup_root.join(Path::new(&rel_path))
    } else {
        // We just read the file here, we know
        // it has a parent.
        song_path.parent().unwrap().join(Path::new(&sample_path))
    }
}

fn on_file_blob(cwd: &Path, path: &Path, data: Vec<u8>) -> Result<HashMap<String, Vec<usize>>, M8FstoErr> {
    let mut reader = Reader::new(data);
    let song = m8_files::Song::
        read_from_reader(&mut reader)
        .map_err(|e| M8FstoErr::UnparseableM8File {
            path: path.to_path_buf(),
            reason: format!("{:?}", e)
        })?;

    let mut missings = HashMap::new();

    for (i, instr) in song.instruments.iter().enumerate() {
        match instr {
            Instrument::Sampler(sampler) if sampler.sample_path.len() > 0 => {
                let full_sample_path =
                    sample_to_absolute_path(cwd, path, &sampler.sample_path );

                if !full_sample_path.exists() {
                    match missings.entry(sampler.sample_path.clone()) {
                        Entry::Vacant(ve) => {
                            ve.insert(vec![i]);
                        }
                        Entry::Occupied(mut o) => {
                            o.get_mut().push(i);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(missings)
}

/// Try to list sample of a given path
pub fn find_broken_sample(cwd: &Path) -> Result<(), M8FstoErr>{
    let pattern = cwd.join("**").join("*.m8s")
        .as_os_str()
        .to_str()
        .map_or(Err(M8FstoErr::InvalidPath { reason: "Invalid pattern".into() }), |v| Ok(v))?
        // .replace('\\', "/")
        .to_string();

    let files = glob::glob(&pattern)
        .map_err(|e|
            M8FstoErr::InvalidSearchPattern { pattern: format!("{:?}", e) })?;

    let mut errors = vec![];
    for entry in files {
        match entry {
            Err(_) => {}
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
                            Ok(result) if result.len() == 0 => {
                            }
                            Ok(result) => {
                                println!("== Broken song {:?}", &path);
                                for (path, instrs) in result.iter() {
                                    print!(" * '{}' in instruments [", path);
                                    for i in instrs {
                                        print!("{}, ", i)
                                    }
                                    println!("]")
                                }
                            }
                            Err(e) => {
                                errors.push(e);
                            }
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
