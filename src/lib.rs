extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate json;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


pub mod client;
pub mod paths;
pub mod error;
pub mod request;
pub mod response;
pub mod resource;

pub mod bucket;
pub mod collection;
pub mod record;

pub mod utils;

pub use error::KintoError;
pub use client::KintoClient;
