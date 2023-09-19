use std::fmt::{self, Display};
use std::error::{Error};

#[derive(Debug)]
pub enum TesseractError{
    TesseractInitError,
    NoSuchFileException,
    TesseracRuntimeError,
    TesseractTimeoutError
}

impl Display for TesseractError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TesseractError::TesseractInitError => write!(f, "Tesseract Init Error"),
            TesseractError::NoSuchFileException => write!(f, "No such file"),
            TesseractError::TesseracRuntimeError => write!(f, "Tesseract Runtime Error"),
            TesseractError::TesseractTimeoutError => write!(f, "Tesseract Runtime Error")
        }
    }
}

impl Error for TesseractError {}
