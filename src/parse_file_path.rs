use thiserror::Error;

#[derive(Debug, Error)]
#[error("Please provide a file path! Usage ./feed-transformer <PATH_TO_FILE>")]
pub struct ParseFileError;

pub fn parse_file_path() -> Result<String, ParseFileError> {
    let mut args = std::env::args();
    args.next(); // Skip the name of the binary being run
    args.next().ok_or(ParseFileError)
}
