use clap::{Parser, Subcommand};

mod ls_sample;
mod grep_sample;
mod bundle;

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
        /// will be in the same current directory.
        out_folder: Option<String>
    }
}

fn main() {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().unwrap();

    match cli.command {
        None => {
            println!("Please use a command")
        }
        Some(M8Commands::LsSample { path }) => {
            ls_sample::ls_sample(cwd.as_path(), &path);
        }
        Some(M8Commands::GrepSample { pattern, path }) => {
            grep_sample::grep_sample(cwd.as_path(), &pattern, &path);
        }
        Some(M8Commands::Bundle { song, out_folder }) => {
            bundle::bundle_song(cwd.as_path(), &song, &out_folder);
        }
    }
}