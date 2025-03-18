use crate::{constant::XML_REG_EXP, parse::HwpxParser, text::Text, Result};
use std::{
    fs::File,
    io::{Read, Seek},
    path::Path,
};
use zip::read::ZipFile;

/// Main container of hwpx file.
pub struct HwpxFile<R: Read + Seek> {
    archive: zip::ZipArchive<R>,
    xmls: Vec<String>,
}

impl<R: Read + Seek> HwpxFile<R> {
    pub fn new(raw: R) -> Result<Self> {
        let archive = zip::ZipArchive::new(raw)?;
        let mut hf = Self {
            archive,
            xmls: Vec::new(),
        };
        hf.collect_xmls()?;
        Ok(hf)
    }

    pub fn xmls(self) -> Vec<String> {
        self.xmls
    }

    pub fn tokenized_xmls(self) -> Vec<Vec<Text>> {
        self.xmls
            .iter()
            .map(|xml| HwpxParser::from_str(xml).collect::<Vec<Text>>())
            .collect()
    }

    fn collect_xmls(&mut self) -> Result<()> {
        for idx in 0..self.archive.len() {
            let mut file = self.archive.by_index(idx)?;
            if is_content(&file) {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                self.xmls.push(content);
            }
        }

        Ok(())
    }
}

impl HwpxFile<File> {
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::new(File::open(path)?)
    }
}

// region: Module functions
fn is_content(file: &ZipFile<'_>) -> bool {
    XML_REG_EXP.is_match(file.name())
}
// endregion

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    macro_rules! test_tokenizing_hwpx {
        ($file_name: literal, $expected: expr) => {
            let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test")
                .join($file_name);

            let hwpx =
                HwpxFile::from_file_path(file_path).expect("File with given name doesn't exist.");

            let tokenized = hwpx.tokenized_xmls();
            for xml_idx in 0..tokenized.len() {
                let xml_texts = &tokenized[xml_idx];
                let expected: &Vec<&str> = &$expected[xml_idx];
                for text_idx in 0..xml_texts.len() {
                    assert_eq!(xml_texts[text_idx].to_string(), expected[text_idx]);
                }
            }
        };
    }

    #[test]
    fn empty_hwpx() {
        test_tokenizing_hwpx!("empty.hwpx", vec![vec![""]]);
    }
}
