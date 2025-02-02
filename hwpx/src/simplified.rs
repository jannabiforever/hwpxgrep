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

#[derive(Clone, Copy)]
pub enum TextKind {
    T,
    Script,
}

pub struct Text {
    pub kind: TextKind,
    pub inner: String,
}

impl Text {
    fn push_str(&mut self, value: &str) {
        match self.kind {
            TextKind::T => self.inner.push_str(value),
            TextKind::Script => self.inner.push_str(value),
        }
    }

    fn from_kind(kind: TextKind) -> Self {
        Self {
            kind,
            inner: String::new(),
        }
    }
}

impl<'a> Iterator for SimplifiedHwpx<'a> {
    type Item = Result<Text>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut text: Option<Text> = None;

        while let Ok(e) = self.reader.next() {
            match e {
                EndDocument { .. } => break,
                StartElement { name, .. } => match name.local_name.as_str() {
                    "t" => {
                        if text.is_some() {
                            return unallowed_event("Nested texts");
                        }
                        text = Some(Text::from_kind(TextKind::T))
                    }
                    "script" => {
                        if text.is_some() {
                            return unallowed_event("Nested texts");
                        }
                        text = Some(Text::from_kind(TextKind::Script))
                    }
                    _ => continue,
                },
                EndElement { name } => match name.local_name.as_str() {
                    "t" | "script" => {
                        return Some(
                            text.ok_or(anyhow!("End of text was found before start was found.")),
                        )
                    }
                    _ => continue,
                },
                Characters(c) => {
                    if let Some(text) = text.as_mut() {
                        text.push_str(&c);
                    }
                }
                _ => continue,
            }
        }

        None
    }
}

// region: module functions
fn unallowed_event(event: &str) -> Option<Result<Text>> {
    Some(Err(anyhow!("{event} are not allowed")))
}
// endregion
