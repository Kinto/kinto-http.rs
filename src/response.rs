use hyper::status::StatusCode;
use hyper::header::Headers;
use serde_json::Value;

use KintoClient;


/// Wrapper for a Kinto response object.
#[derive(Debug, Clone)]
pub struct ResponseWrapper {
    pub client: KintoClient,
    pub path: String,
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Value,
}
