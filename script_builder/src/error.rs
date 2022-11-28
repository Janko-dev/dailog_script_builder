use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum DailogError<'a>{
    ParseError(&'a str),
}

impl<'a> fmt::Display for DailogError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dailog JSON Parsing Error")
    }
}

impl<'a> Error for DailogError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
