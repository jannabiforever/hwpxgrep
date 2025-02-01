use anyhow::Result;
use image::DynamicImage;
use regex::Regex;
use std::{
    io::{Read, Seek},
    path::Path,
};
use zip::read::ZipFile;

lazy_static::lazy_static! {
    static ref XML_REG_EXP: Regex = Regex::new(r"Contents/section\d+\.xml").unwrap(); // No need to handle errors here.
    static ref IMAGE_REG_EXP: Regex = Regex::new(r"BinData/image\d+\.(?i:jpg|bmp)").unwrap(); // No need to handle errors here.
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

impl HwpxFile<std::fs::File> {
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::new(std::fs::File::open(path)?)
    }
}

/// Necessary hwpx file content.
/// Only should be created through `HwpxFile::extract`.
#[derive(Debug)]
pub struct Extracted {
    pub xmls: Vec<String>,
    pub images: Vec<DynamicImage>,
}

impl Extracted {
    fn new() -> Self {
        Self {
            xmls: Vec::new(),
            images: Vec::new(),
        }
    }

    fn push_content(&mut self, content: String) {
        self.xmls.push(content)
    }

    fn push_image(&mut self, image: DynamicImage) {
        self.images.push(image)
    }
}

// TODO: Implement this
pub fn save_extracted<P: AsRef<Path>>(_: &Extracted, _: P) -> anyhow::Result<()> {
    Ok(())
}

// region: Module functions
fn try_get_file(file: &mut ZipFile, res: &mut Extracted) -> Result<()> {
    if is_content(file.name()) {
        res.push_content(get_content(file)?);
    } else if is_image(file.name()) {
        res.push_image(get_image(file)?);
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

fn is_image(name: &str) -> bool {
    IMAGE_REG_EXP.is_match(name)
}

fn get_image(file: &mut ZipFile) -> Result<DynamicImage> {
    let mut raw_image = Vec::new();
    file.read_to_end(&mut raw_image)?;
    let image = image::load_from_memory(&raw_image)?;

    Ok(image)
}
// endregion
