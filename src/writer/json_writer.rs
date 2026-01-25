use crate::writer::{WriteError, Writer};
use serde::Serialize;
use std::{fs::File, io::BufWriter};
use struson::writer::{JsonStreamWriter, JsonWriter as _, WriterSettings};

pub struct JsonWriter {
    writer: JsonStreamWriter<BufWriter<File>>,
}

impl Writer for JsonWriter {
    fn new(path: &str) -> Result<Self, WriteError> {
        let file = File::create(path)?;
        let buffer = BufWriter::new(file);
        let json_settings = WriterSettings {
            pretty_print: true,
            ..Default::default()
        };
        let writer = JsonStreamWriter::new_custom(buffer, json_settings);

        Ok(Self { writer })
    }

    fn start(&mut self) -> Result<(), WriteError> {
        self.writer.begin_object()?;
        self.writer.name("events")?;
        self.writer.begin_array()?;
        Ok(())
    }

    fn add(&mut self, event: &impl Serialize) -> Result<(), WriteError> {
        self.writer
            .serialize_value(event)
            .map_err(|e| WriteError::SerializeError(e.to_string()))?;
        Ok(())
    }

    fn end(mut self) -> Result<(), WriteError> {
        self.writer.end_array()?;
        self.writer.end_object()?;

        self.writer.finish_document()?;
        Ok(())
    }
}
