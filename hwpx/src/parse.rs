use crate::{error::HwpxError::*, stream::XmlEventStream, text::Text, Result};
use std::iter::Peekable;
use xml::reader::XmlEvent::{self as XE, *};

/// Hwpx xml struct that modifies itself in simplified form.
pub struct HwpxParser<I>
where
    I: Iterator<Item = XE>,
{
    stream: Peekable<I>,
}

impl<I> HwpxParser<I>
where
    I: Iterator<Item = XE>,
{
    pub fn new(iterator: I) -> Self {
        Self {
            stream: iterator.peekable(),
        }
    }

    fn visit_t(&mut self) -> Result<Text> {
        self.stream.next(); // Consume StartElement.
        let mut collected = Vec::new();
        loop {
            if let Some(e) = self.stream.peek() {
                match e {
                    XE::Characters(s) => collected.push(s.clone()),
                    XE::EndElement { name } if name.local_name.as_str() == "t" => break,
                    e => return Err(UnexpectedEvent((*e).clone())),
                }

                self.stream.next();
            }
        }
        let inner = collected.into_iter().collect::<String>();
        Ok(Text::new_t(inner))
    }

    fn visit_script(&mut self) -> Result<Text> {
        self.stream.next(); // Consume StartElement.
        let mut collected = Vec::new();
        loop {
            if let Some(e) = self.stream.peek() {
                match e {
                    XE::Characters(s) => collected.push(s.clone()),
                    XE::EndElement { name } if name.local_name.as_str() == "script" => break,
                    e => return Err(UnexpectedEvent((*e).clone())),
                }

                self.stream.next();
            }
        }
        let inner = collected.into_iter().collect::<String>();
        Ok(Text::new_t(inner))
    }
}

impl<I> Iterator for HwpxParser<I>
where
    I: Iterator<Item = XE>,
{
    type Item = Text;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(e) = self.stream.peek() {
            match e {
                EndDocument { .. } => break,
                StartElement { name, .. } => match name.local_name.as_str() {
                    "t" => return self.visit_t().ok(),
                    "script" => return self.visit_script().ok(),
                    _ => continue,
                },
                _ => continue,
            }
        }
        None
    }
}

impl<'a> HwpxParser<XmlEventStream<&'a [u8]>> {
    pub fn from_str(value: &'a str) -> Self {
        let stream = XmlEventStream::from_str(value);
        Self::new(stream)
    }
}
