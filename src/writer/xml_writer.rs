use crate::writer::{WriteError, Writer};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::writer::Writer as QuickXmlWriter;
use serde::Serialize;
use std::io::Write;
use std::{fs::File, io::BufWriter};

pub struct XmlWriter {
    writer: QuickXmlWriter<BufWriter<File>>,
}

impl Writer for XmlWriter {
    fn new(path: &str) -> Result<Self, WriteError> {
        let file = File::create(path)?;
        let buffer = BufWriter::new(file);
        let writer = QuickXmlWriter::new_with_indent(buffer, b' ', 2);

        Ok(Self { writer })
    }

    fn start(&mut self) -> Result<(), WriteError> {
        self.writer
            .write_event(Event::Start(BytesStart::new("inventory")))?;

        self.writer
            .write_event(Event::Start(BytesStart::new("events")))?;

        Ok(())
    }

    fn add(&mut self, object: &impl Serialize) -> Result<(), WriteError> {
        self.writer
            .write_serializable("event", object)
            .map_err(|e| WriteError::SerializeError(e.to_string()))?;

        Ok(())
    }

    fn end(mut self) -> Result<(), WriteError> {
        self.writer
            .write_event(Event::End(BytesEnd::new("events")))?;

        self.writer
            .write_event(Event::End(BytesEnd::new("inventory")))?;

        self.writer.get_mut().flush()?;

        Ok(())
    }
}
