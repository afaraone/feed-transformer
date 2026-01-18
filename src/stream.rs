use crate::event::TmEvent;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use struson::json_path;
use struson::reader::{JsonReader, JsonStreamReader};

pub struct StreamIterator {
    stream: JsonStreamReader<BufReader<File>>,
}

impl Iterator for StreamIterator {
    type Item = Result<TmEvent, Box<dyn std::error::Error>>;

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

impl StreamIterator {
    pub fn new(file_path: &str, top_level_key: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let buffer = BufReader::new(file);
        let mut stream = JsonStreamReader::new(buffer);
        stream.seek_to(&json_path![top_level_key])?;
        stream.begin_array()?;

        Ok(StreamIterator { stream })
    }
}
