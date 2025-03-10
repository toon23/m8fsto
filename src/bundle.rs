use std::{collections::HashMap, fs, path::Path};
use glob::glob;
use m8_files::{reader::*, Instrument};

fn on_file_blob(_cwd: &Path, _path: &Path, data: Vec<u8>) {
    let mut reader = Reader::new(data);
    let may_song = m8_files::Song::read_from_reader(&mut reader);
    let mut samples = HashMap::new();

    match may_song {
        Err(_) => {

        }
        Ok(song) => {
            for (i, instr) in song.instruments.iter().enumerate() {
                match instr {
                    Instrument::Sampler(sampler) => {
                        samples.insert(sampler.sample_path.clone(), i);
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
pub fn bundle_song(cwd: &Path, path : &str, _out_folder: &Option<String>) {
    let try_as_file = fs::read(path);
    match try_as_file {
        Err(_) => { on_dir(cwd, path) }
        Ok(file_blob) => {
            let as_path = Path::new(path);
            on_file_blob(cwd, as_path, file_blob);
        }
    }
}