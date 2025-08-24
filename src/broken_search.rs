use std::{collections::hash_map::Entry, path::PathBuf};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use m8_file_parser::{reader::*, Instrument};

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
    let song = m8_file_parser::Song::
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

/// Recursively search a directory for song files and report broken samples
pub fn find_broken_samples_under_dir(cwd: &Path) -> Result<(), M8FstoErr>{
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
    let cwd = cwd.to_path_buf();
    for entry in files {
        match entry {
            Err(_) => {}
            Ok(path) => {
                if let Err(e) = find_broken_sample_in_song(&cwd, path) {
                    errors.push(e);
                }

            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors[0].clone())
    } else {
        Err(M8FstoErr::MultiErrs { inner: errors })
    }
}


/// Report broken samples in a single `.m8s` song file.
pub fn find_broken_sample_in_song(backup_root : &PathBuf, song_path: PathBuf) -> Result<(), M8FstoErr> {
    let file_blob = fs::read(&song_path).map_err(|e| M8FstoErr::CannotReadFile {
        path: song_path.clone(),
        reason: format!("{:?}", e),
    })?;

    match on_file_blob(&backup_root, &song_path, file_blob) {
        Ok(result) if result.is_empty() => Ok(()),
        Ok(result) => {
            println!("== Broken song {:?}", &song_path);
            for (sample_path, instrs) in result.iter() {
                print!(" * '{}' in instruments [", sample_path);
                for i in instrs {
                    print!("{}, ", i)
                }
                println!("]")
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}


/// Report broken song samples in a list of directories and/or song paths
pub fn process_paths(cwd: &Path, paths: &[String]) -> Result<(), M8FstoErr> {
    let mut roots = Vec::new();
    let mut songs = Vec::new();

    if paths.is_empty() {
       roots.push(cwd.to_path_buf());
    }

    let mut errors = vec![];
    for path in paths {
        let path_buf = PathBuf::from(path);
        if path_buf.is_dir() {
            roots.push(path_buf);
        } else if path_buf.is_file() && path.ends_with(".m8s") {
            songs.push(path_buf);
        } else {
            errors.push(M8FstoErr::InvalidSearchPattern { pattern: path.to_string()});
            eprintln!("Warning: Ignoring invalid path: {}", path);
        }
    }

    for root in roots {
        if let Err(e) = find_broken_samples_under_dir(root.as_path()) {
            errors.push(e);
        }
    }

    let cwd = &cwd.to_path_buf();
    for song in songs {
        if let Err(e) = find_broken_sample_in_song(cwd, song) {
            errors.push(e);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors[0].clone())
    } else {
        Err(M8FstoErr::MultiErrs { inner: errors })
    }
}
