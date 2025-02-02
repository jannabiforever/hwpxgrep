use anyhow::Result;
use regex::Regex;
use std::{
    fs::{self, File},
    io::{Read, Seek, Write},
    path::Path,
};
use zip::read::ZipFile;

lazy_static::lazy_static! {
    static ref XML_REG_EXP: Regex = Regex::new(r"Contents/section\d+\.xml").unwrap(); // No need to handle errors here.
}

/// Main container of hwpx file.
pub struct HwpxFile<R: Read + Seek> {
    archive: zip::ZipArchive<R>,
}

impl<R: Read + Seek> HwpxFile<R> {
    pub fn new(raw: R) -> Result<Self> {
        let archive = zip::ZipArchive::new(raw)?;
        Ok(Self { archive })
    }

    pub fn extract(mut self) -> Result<Extracted> {
        let mut extracted = Extracted::new();
        for idx in 0..self.archive.len() {
            try_get_file(&mut self.archive.by_index(idx)?, &mut extracted)?;
        }
        Ok(extracted)
    }
}

impl HwpxFile<File> {
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::new(File::open(path)?)
    }
}

/// Necessary hwpx file content.
/// Only should be created through `HwpxFile::extract`.
#[derive(Debug)]
pub struct Extracted {
    pub xmls: Vec<String>,
}

impl Extracted {
    fn new() -> Self {
        Self { xmls: Vec::new() }
    }

    fn push_content(&mut self, content: String) {
        self.xmls.push(content)
    }
}

/// Save extracted datum in given path.
pub fn save_extracted<P: AsRef<Path>>(data: &Extracted, path: P) -> anyhow::Result<()> {
    let path = path.as_ref().join("xmls");
    fs::create_dir_all(&path)?;

    for (idx, xml) in data.xmls.iter().enumerate() {
        let mut file = File::create(path.join(format!("{idx}.xml")))?;
        file.write_all(xml.as_bytes())?;
    }

    Ok(())
}

// region: Module functions
fn try_get_file(file: &mut ZipFile, res: &mut Extracted) -> Result<()> {
    if is_content(file.name()) {
        res.push_content(get_content(file)?);
    }

    Ok(())
}

fn is_content(name: &str) -> bool {
    XML_REG_EXP.is_match(name)
}

fn get_content(file: &mut ZipFile) -> Result<String> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
// endregion
