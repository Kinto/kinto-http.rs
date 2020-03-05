extern crate hyper;
extern crate serde;

use std::io::Error as IOError;
use std::str::Utf8Error;
use hyper::Error as HyperError;
use serde_json::error::Error as JsonError;


#[derive(Debug)]
pub enum KintoError {
    NotModified,
    PreconditionError,
    UndefinedIdError,
    UnavailableEndpointError,
    HyperError,
    JsonError(JsonError),
    Utf8Error(Utf8Error),
    IOError(IOError),
}


impl From<IOError> for KintoError {
    fn from(err: IOError) -> Self {
        KintoError::IOError(err)
    }
}


impl From<Utf8Error> for KintoError {
    fn from(err: Utf8Error) -> Self {
        KintoError::Utf8Error(err)
    }
}


impl From<JsonError> for KintoError {
    fn from(err: JsonError) -> Self {
        KintoError::JsonError(err)
    }
}


impl From<HyperError> for KintoError {
    fn from(err: HyperError) -> Self {
        KintoError::HyperError
    }
}
