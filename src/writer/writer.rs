use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),

    #[error("SerializeError: {0}")]
    SerializeError(String),
}

pub trait Writer: Sized {
    fn new(path: &str) -> Result<Self, WriteError>;
    fn start(&mut self) -> Result<(), WriteError>;
    fn add(&mut self, object: &impl Serialize) -> Result<(), WriteError>;
    fn end(self) -> Result<(), WriteError>;
}
