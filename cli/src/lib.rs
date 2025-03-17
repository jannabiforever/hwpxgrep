use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: UtilCommand,
}

#[derive(Subcommand, Debug)]
enum UtilCommand {
    /// Extract hwpx to given path
    Extract {
        /// The path of hwpx file
        hwpx_path: PathBuf,
        #[arg(long, short)]
        folder_path: Option<PathBuf>,
    },
    Index {
        folder_path: Option<PathBuf>,
    },
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        UtilCommand::Extract {
            hwpx_path,
            folder_path,
        } => {
            println!("extracted {:?} to {:?}", hwpx_path, folder_path);
        }
        UtilCommand::Index { folder_path } => {
            println!("Indexing under {:?}", folder_path);
        }
    }
}
