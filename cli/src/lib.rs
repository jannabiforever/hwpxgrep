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
        #[arg(short, long)]
        hwpx_path: String,
        #[arg(long)]
        folder_path: Option<String>,
    },
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        UtilCommand::Extract {
            hwpx_path: _,
            folder_path: _,
        } => todo!(),
    }
}
