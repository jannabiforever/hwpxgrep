use anyhow::{anyhow, Result};
use xml::{reader::XmlEvent::*, EventReader};

/// Hwpx xml struct that modifies itself in simplified form.
pub struct SimplifiedHwpx<'a> {
    reader: EventReader<&'a [u8]>,
}

impl<'a> SimplifiedHwpx<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self {
            reader: EventReader::from_str(raw),
        }
    }
}

pub enum TextType {
    T,
    Script,
}

impl TextType {
    /// TextType from element name.
    /// panics when unallowed node name is given.
    fn new(name: &str) -> Self {
        match name {
            "t" => Self::T,
            "script" => Self::Script,
            _ => unreachable!("TextType is only allowed for t and script."),
        }
    }
}

/// Simplified events.
/// SimplifiedHwpx iterates raw xml by these events.
pub enum SimplifiedHwpxEvent {
    /// Node that only has characters in itself.
    Text(TextType, String),
    /// Image node. It contains the image file stem in it.
    Image(String),
    /// Table node.
    TableStart {
        row_cnt: usize,
        col_cnt: usize,
    },
    TableEnd,
    Cell,
}

impl<'a> Iterator for SimplifiedHwpx<'a> {
    type Item = Result<SimplifiedHwpxEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut text: Option<(TextType, String)> = None;

        while let Ok(e) = self.reader.next() {
            match e {
                EndDocument { .. } => break,
                StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    c if ["t", "script"].contains(&c) => {
                        if text.is_some() {
                            return unallowed_event("Nested texts");
                        }
                        text = Some((TextType::new(c), String::new()));
                    }
                    "tbl" => return table_start_event(&attributes),
                    "tc" => return cell_event(),
                    "img" => return image_event(&attributes),
                    _ => continue,
                },
                EndElement { name } => match name.local_name.as_str() {
                    "t" | "script" => return text_event(text),
                    "tbl" => return table_end_event(),
                    _ => continue,
                },
                Characters(c) => {
                    if let Some((_, s)) = text.as_mut() {
                        s.push_str(&c);
                    }
                }
                _ => continue,
            }
        }

        None
    }
}

// -- module functions
fn get_attribute(attributes: &[xml::attribute::OwnedAttribute], name: &str) -> Option<String> {
    attributes
        .iter()
        .find(|&a| a.name.local_name == name)
        .map(|a| a.value.clone())
}

fn get_table_dimension(attributes: &[xml::attribute::OwnedAttribute]) -> Option<(usize, usize)> {
    let row_cnt = get_attribute(attributes, "rowCnt").and_then(|e| e.parse().ok());
    let col_cnt = get_attribute(attributes, "colCnt").and_then(|e| e.parse().ok());

    row_cnt.zip(col_cnt)
}

fn table_start_event(
    attributes: &[xml::attribute::OwnedAttribute],
) -> Option<Result<SimplifiedHwpxEvent>> {
    if let Some((row_cnt, col_cnt)) = get_table_dimension(attributes) {
        return Some(Ok(SimplifiedHwpxEvent::TableStart { row_cnt, col_cnt }));
    } else {
        return Some(Err(anyhow::anyhow!(
            "Current table node doesn't have dimension attributes"
        )));
    }
}

fn cell_event() -> Option<Result<SimplifiedHwpxEvent>> {
    Some(Ok(SimplifiedHwpxEvent::Cell))
}

fn image_event(
    attributes: &[xml::attribute::OwnedAttribute],
) -> Option<Result<SimplifiedHwpxEvent>> {
    match get_attribute(&attributes, "binaryItemIDRef") {
        Some(s) => {
            return Some(Ok(SimplifiedHwpxEvent::Image(s)));
        }
        None => {
            return Some(Err(anyhow!(
                "Current image node doesn't have source attributes"
            )));
        }
    }
}

fn text_event(text: Option<(TextType, String)>) -> Option<Result<SimplifiedHwpxEvent>> {
    match text {
        None => Some(Err(anyhow::anyhow!(
            "Found end of a text node while not reading one."
        ))),
        Some((t, s)) => Some(Ok(SimplifiedHwpxEvent::Text(t, s))),
    }
}

fn table_end_event() -> Option<Result<SimplifiedHwpxEvent>> {
    Some(Ok(SimplifiedHwpxEvent::TableEnd))
}

fn unallowed_event(event: &str) -> Option<Result<SimplifiedHwpxEvent>> {
    Some(Err(anyhow!("{event} are not allowed")))
}
