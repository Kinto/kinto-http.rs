extern crate hyper;
extern crate serde;

use std::io::Error as IOError;
use hyper::Error as HyperError;
use serde_json::error::Error as JsonError;


#[derive(Debug)]
pub enum KintoError {
    NotModified,
    PreconditionError,
    UndefinedIdError,
    HyperError,
    JsonError,
    IOError,
}


impl From<IOError> for KintoError {
    fn from(err: IOError) -> Self {
        err.into()
    }
}


impl From<JsonError> for KintoError {
    fn from(err: JsonError) -> Self {
        err.into()
    }
}


impl From<HyperError> for KintoError {
    fn from(err: HyperError) -> Self {
        err.into()
    }
}
