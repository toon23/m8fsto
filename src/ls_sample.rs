use std::{fs, path::Path};
use glob::glob;
use m8_files::{reader::*, Instrument};

fn on_file_blob(cwd: &Path, path: &Path, data: Vec<u8>) {
    let mut reader = Reader::new(data);
    let may_song = m8_files::Song::read_from_reader(&mut reader);
    match may_song {
        Err(_) => {}
        Ok(song) => {
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
        }
    }
}

fn on_dir(cwd: &Path, path: &str) {
    for entry in glob(path).expect("Failed to read glob pattern") {
        match entry {
            Err(e) => println!("{:?}", e),
            Ok(path) => {
                let try_as_file = fs::read(&path);
                match try_as_file {
                    Err(_) => {}
                    Ok(file_blob) => {
                        on_file_blob(cwd, path.as_path(), file_blob);
                    }
                }
            }
        }
    }
}

/// Try to list sample of a given path
pub fn ls_sample(cwd: &Path, path : &Option<String>) {
    match path {
        None => on_dir(cwd, "./"),
        Some(path) => {
            let try_as_file = fs::read(path);
            match try_as_file {
                Err(_) => { on_dir(cwd, path) }
                Ok(file_blob) => {
                    let as_path = Path::new(path);
                    on_file_blob(cwd, as_path, file_blob);
                }
            }
        }
    }
}