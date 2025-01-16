use anyhow::Result;
use image::DynamicImage;
use regex::Regex;
use std::{
    io::{Read, Seek, Write},
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

    /// Save the extracted files under given folder path.
    /// Fails and return Err when given path is not directory, or already exists.
    pub fn save<P: AsRef<Path>>(self, path: P) -> Result<()> {
        Self::validate_dir(path.as_ref())?;
        Self::create_dir(path.as_ref())?;
        self.save_xmls(path.as_ref())?;
        self.save_images(path.as_ref())?;
        Ok(())
    }

    fn validate_dir(path: &Path) -> Result<()> {
        anyhow::ensure!(
            !path.exists(),
            "Couldn't save cache file because given base path already exists"
        );

        Ok(())
    }

    fn create_dir(path: &Path) -> Result<()> {
        std::fs::create_dir(path)?;
        std::fs::create_dir(path.join("images"))?;
        std::fs::create_dir(path.join("xmls"))?;
        Ok(())
    }

    fn save_xmls(&self, path: &Path) -> Result<()> {
        for (idx, xml) in self.xmls.iter().enumerate() {
            let mut file = std::fs::File::create(path.join(format!("xmls/{}.xml", idx)))?;
            file.write_all(xml.as_bytes())?;
        }

        Ok(())
    }

    fn save_images(&self, path: &Path) -> Result<()> {
        for (idx, image) in self.images.iter().enumerate() {
            // TODO: we might need more supports.
            image.save_with_format(
                path.join(format!("images/{}.jpg", idx)),
                image::ImageFormat::Jpeg,
            )?;
        }
        Ok(())
    }

    fn push_content(&mut self, content: String) {
        self.xmls.push(content)
    }

    fn push_image(&mut self, image: DynamicImage) {
        self.images.push(image)
    }
}

// -- Module functions
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
