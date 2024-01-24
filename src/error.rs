use std::io;

pub type SortResult<T> = Result<T, SortError>;

#[derive(Debug)]
pub enum SortError {
    Error(String),
    IOError(String)
}

impl From<io::Error> for SortError {
    fn from(value: io::Error) -> Self {
        
        Self::IOError(value.to_string())
    }
}

impl From<String> for SortError {

    fn from(value: String) -> Self {

        Self::Error(value)
    }
}