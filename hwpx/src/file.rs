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

    /// It is for extracting xmls from hwpx file to see the content.
    /// TODO: so, it might wants to be prettified.
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
fn is_content<R: Read>(file: &ZipFile<'_, R>) -> bool {
    XML_REG_EXP.is_match(file.name())
}
// endregion

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn convert_to_string<T: std::fmt::Display>(vec: Vec<Vec<T>>) -> Vec<Vec<String>> {
        vec.iter()
            .map(|texts| {
                texts
                    .iter()
                    .map(|text| text.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
    }

    macro_rules! test_tokenizing_hwpx {
        ($file_name: literal, $expected: expr) => {
            let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test")
                .join($file_name);

            let hwpx =
                HwpxFile::from_file_path(file_path).expect("File with given name doesn't exist.");

            let tokenized = convert_to_string(hwpx.tokenized_xmls());
            let expected = convert_to_string($expected);

            assert_eq!(tokenized, expected);
        };
    }

    #[test]
    fn empty_hwpx() {
        test_tokenizing_hwpx!("empty.hwpx", vec![vec![""]]);
    }

    #[test]
    fn complex_hwpx() {
        test_tokenizing_hwpx!(
            "complex.hwpx",
            vec![vec!["그림과 같이 한 변의 길이가 ", "1`", "인 정사각형 ", "rmABCD`", "가 있다. ", "0 < t< {sqrt{2}}over{2}`", "인 실수 ", "t`", "에 대하여 점 ", "rmP`", "를 ", "rm {bar{AP}}`=it t`", "를 만족시키는 ", "선분 ", "rmAC", " 위의 점이라 하고, 직선 ", "rmBP`", "가 선분 ", "rmAD`", "와 만나는 점을 ", "rmQ`", "라 하자. ", "사각형 ", "rmCDQP`", "의 넓이를 ", "S` LEFT ( t RIGHT )`", "라 할 때, ", "lim_{t`rarrow` { sqrt{2}} over { 2}- } { { S` LEFT ( t RIGHT ) } over { 1- sqrt{2}t } }`", "의 값은?", "", "① ", "②  ", "③  ", "④  ", "⑤  ", "", "① ", "② ", "③ ", "④ ", "⑤ ", "", "Y431", "출제자", "유정인", "정답", "3 / 4", "사용", "비고", "확인", "", "나눔스퀘어, 글씨크기10, 수식10(EQ), 장평100, 자간–8 반드시 지켜주세요. ", ""],]
        );
    }
}
