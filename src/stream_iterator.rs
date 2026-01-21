use crate::tm_event::TmEvent;
use std::fs::File;
use std::io::{self, BufReader};
use struson::json_path;
use struson::reader::{self, JsonReader, JsonStreamReader};
use struson::serde;
use thiserror::Error;

pub struct StreamIterator {
    stream: JsonStreamReader<BufReader<File>>,
}

impl Iterator for StreamIterator {
    type Item = Result<TmEvent, StreamError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.has_next() {
            Ok(true) => match self.stream.deserialize_next::<TmEvent>() {
                Ok(event) => Some(Ok(event)),
                Err(e) => Some(Err(e.into())),
            },
            Ok(false) => {
                let _ = self.stream.end_array();
                None
            }
            Err(e) => Some(Err(e.into())),
        }
    }
}

const TOP_LEVEL_KEY: &str = "events";

impl StreamIterator {
    pub fn new(file_path: &str) -> Result<Self, StreamError> {
        let file = File::open(file_path)?;
        let buffer = BufReader::new(file);
        let mut stream = JsonStreamReader::new(buffer);
        stream.seek_to(&json_path![TOP_LEVEL_KEY])?;
        stream.begin_array()?;

        Ok(StreamIterator { stream })
    }
}

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("Failed to find top-level array in JSON - {0}")]
    JsonFindArrayError(#[from] reader::ReaderError),

    #[error("Deserialize JSON error - {0}")]
    JsonDeserializeError(#[from] serde::DeserializerError),

    #[error("IO Error - {0}")]
    IoError(#[from] io::Error),
}
