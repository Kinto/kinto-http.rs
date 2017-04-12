use hyper::status::StatusCode;
use hyper::header::Headers;
use serde_json::Value;

use KintoConfig;


/// Wrapper for a Kinto response object.
#[derive(Debug, Clone)]
pub struct ResponseWrapper {
    pub config: KintoConfig,
    pub path: String,
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Value,
}
