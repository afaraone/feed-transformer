use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::writer::Writer as XmlWriter;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use thiserror::Error;

pub struct InventoryWriter {
    writer: XmlWriter<BufWriter<File>>,
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("Failed to write inventory - {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to serialize event: {0}")]
    SerializeError(#[from] quick_xml::SeError),
}

impl InventoryWriter {
    pub fn new(path: &str) -> Result<Self, WriteError> {
        let file = File::create(path)?;
        let buffer = BufWriter::new(file);
        let writer = XmlWriter::new_with_indent(buffer, b' ', 2);

        Ok(Self { writer })
    }

    // creates <inventory><events> tags
    pub fn start(&mut self) -> Result<(), WriteError> {
        self.writer
            .write_event(Event::Start(BytesStart::new("inventory")))?;

        self.writer
            .write_event(Event::Start(BytesStart::new("events")))?;

        Ok(())
    }

    pub fn add(&mut self, event: &impl Serialize) -> Result<(), WriteError> {
        self.writer.write_serializable("event", event)?;

        Ok(())
    }

    pub fn end(&mut self) -> Result<(), WriteError> {
        self.writer
            .write_event(Event::End(BytesEnd::new("events")))?;

        self.writer
            .write_event(Event::End(BytesEnd::new("inventory")))?;

        self.writer.get_mut().flush()?;

        Ok(())
    }
}
