use thiserror::Error;

#[derive(Debug, Error)]
#[error("Please provide a file path! Usage ./feed-transformer <PATH_TO_FILE>")]
pub struct ParseFileError;

pub fn parse_file_path() -> Result<String, ParseFileError> {
    let mut args = std::env::args();
    args.nth(1).ok_or(ParseFileError)
}
