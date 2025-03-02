use std::fs;
use std::path::Path;
use std::path::PathBuf;
use glob::glob;
use glob::Pattern;
use m8_files::{reader::*, Instrument};

fn on_file_blob(cwd: &Path, pattern: &Pattern, path: &Path, data: Vec<u8>) {
    let mut reader = Reader::new(data);
    let may_song = m8_files::Song::read_from_reader(&mut reader);
    match may_song {
        Err(_) => {}
        Ok(song) => {
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
        }
    }
}

fn on_dir(cwd: &Path, pattern: &Pattern, path: &str) {
    for entry in glob(path).expect("Failed to read glob pattern") {
        match entry {
            Err(e) => println!("{:?}", e),
            Ok(path) => {
                let try_as_file = fs::read(&path);
                match try_as_file {
                    Err(_) => {}
                    Ok(file_blob) => {
                        on_file_blob(cwd, pattern,&path, file_blob);
                    }
                }
            }
        }
    }
}

/// Try to list sample of a given path
pub fn grep_sample(cwd: &Path, pattern: &str, path : &Option<String>) {
    let pat =
        match glob::Pattern::new(pattern) {
            Err(e) => {
                println!("Invalid search pattern {:?}", e);
                return
            }
            Ok(pat) => pat
        };

    match path {
        None => on_dir(cwd, &pat, "./"),
        Some(path) => {
            let try_as_file = fs::read(path);
            match try_as_file {
                Err(_) => { on_dir(cwd, &pat, path) }
                Ok(file_blob) => {
                    let as_path = Path::new(path);
                    on_file_blob(cwd, &pat, as_path, file_blob);
                }
            }
        }
    }
}