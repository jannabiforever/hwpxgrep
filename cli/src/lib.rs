mod config;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    Tokenize {
        hwpx_path: PathBuf,
        folder_path: Option<PathBuf>,
    },
    Cache {
        folder_path: PathBuf,
    },
}

pub fn run() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        UtilCommand::Extract {
            hwpx_path,
            folder_path,
        } => {
            extract::extract(&hwpx_path, &folder_path).unwrap();
            println!("extracted xml from {:?} to {:?}", hwpx_path, folder_path);
        }
        UtilCommand::Tokenize {
            hwpx_path,
            folder_path,
        } => {
            tokenize::tokenize(&hwpx_path, &folder_path).unwrap();
            println!("tokenized xml from {:?} to {:?}", hwpx_path, folder_path);
        }
        UtilCommand::Cache { folder_path } => {
            cache::cache(&folder_path).unwrap();
            println!("Indexing files under folder: {:?}", folder_path);
        }
    }
}

mod extract {
    use hwpx::{error::HwpxError, file::HwpxFile};
    use std::{fs, io::prelude::*, path::PathBuf};

    pub fn extract(hwpx_path: &PathBuf, folder_path: &Option<PathBuf>) -> Result<(), HwpxError> {
        let folder_path = folder_path.clone().unwrap_or_else(|| {
            let mut path = hwpx_path.clone();
            path.set_extension("");
            path
        });

        let xml_contents = HwpxFile::from_file_path(hwpx_path)?.xmls();

        fs::create_dir_all(&folder_path)?;
        for (idx, xml) in xml_contents.iter().enumerate() {
            let mut file = fs::File::create(folder_path.join(format!("{}.xml", idx)))?;
            file.write_all(xml.as_bytes())?;
        }

        Ok(())
    }
}

mod tokenize {
    use hwpx::{error::HwpxError, file::HwpxFile};
    use std::{fs, io::prelude::*, path::PathBuf};

    pub fn tokenize(hwpx_path: &PathBuf, folder_path: &Option<PathBuf>) -> Result<(), HwpxError> {
        let folder_path = folder_path.clone().unwrap_or_else(|| {
            let mut path = hwpx_path.clone();
            path.set_extension("");
            path
        });

        let tokenized_contents = HwpxFile::from_file_path(hwpx_path)?
            .tokenized_xmls()
            .iter()
            .map(|xml| xml.iter().map(|text| text.to_string()).collect::<String>())
            .collect::<Vec<String>>();

        fs::create_dir_all(&folder_path)?;
        for (idx, tokenized) in tokenized_contents.iter().enumerate() {
            let mut file = fs::File::create(folder_path.join(format!("{}-token.txt", idx)))?;
            file.write_all(tokenized.as_bytes())?;
        }

        Ok(())
    }
}

mod cache {
    use super::config::HWPXG_CACHE_FILE;
    use hwpx::error::HwpxError;
    use std::{fs, path::PathBuf};

    pub fn cache(folder_path: &PathBuf) -> Result<(), HwpxError> {
        let mut cache = fs::File::open(&*HWPXG_CACHE_FILE)?;
        todo!()
    }
}
