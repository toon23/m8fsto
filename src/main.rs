use std::path::PathBuf;

use clap::{Parser, Subcommand};
use types::M8FstoErr;

mod ls_sample;
mod grep_sample;
mod bundle;
mod broken_search;
mod types;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
/// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<M8Commands>
}

#[derive(Subcommand)]
enum M8Commands {
    /// List samples used in M8 song file
    LsSample {
        /// Optional path/folder
        path: Option<String>
    },

    /// Try to find songs that are using a given sample
    GrepSample {
        /// Pattern to search, representing a sample file path using
        /// glob patterns
        pattern : String,

        /// In which folder to search
        path : Option<String>
    },

    /// Bundle a song, avoiding sample duplication
    Bundle {
        /// Specific song only
        song : String,

        /// Where to write the bundled song, by default
        /// will be in current directory.
        out_folder: Option<String>
    },

    /// Try to find broken sample path in songs from a given root
    BrokenSearch {
        /// Optional root folder for the sample path, if not
        /// set, current working directory is used.
        root : Option<String>

    }
}

fn print_errors(r : Result<(), M8FstoErr>) {
    match r {
        Ok(()) => {}
        Err(e) => eprintln!("{}", e)
    }
}

fn main() {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().unwrap();

    match cli.command {
        None => { println!("Please use a command") }
        Some(M8Commands::LsSample { path }) => {
            print_errors(ls_sample::ls_sample(cwd.as_path(), &path))
        }
        Some(M8Commands::GrepSample { pattern, path }) => {
            print_errors(grep_sample::grep_sample(cwd.as_path(), &pattern, &path))
        }
        Some(M8Commands::BrokenSearch { root }) => {
            let root = root
                .map_or_else(
                    || cwd.as_path().to_path_buf(),
                     |f| PathBuf::from(f));

            print_errors(broken_search::find_broken_sample(root.as_path()))
        }
        Some(M8Commands::Bundle { song, out_folder }) => {
            print_errors(bundle::bundle_song(cwd.as_path(), &song, &out_folder))
        }
    }
}
