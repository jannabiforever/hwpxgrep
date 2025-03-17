use std::io::Read;
use xml::{
    reader::{Events, XmlEvent},
    EventReader,
};

pub struct XmlEventStream<R: Read> {
    reader: Events<R>,
}

impl<'a> XmlEventStream<&'a [u8]> {
    pub fn from_str(source: &'a str) -> Self {
        Self {
            reader: EventReader::from_str(source).into_iter(),
        }
    }
}

impl<R: Read> Iterator for XmlEventStream<R> {
    type Item = XmlEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next()?.ok()
    }
}
