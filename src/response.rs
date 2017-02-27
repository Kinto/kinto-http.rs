use json::JsonValue;
use hyper::status::StatusCode;
use hyper::header::Headers;

use KintoClient;


/// Wrapper for a Kinto response object.
#[derive(Debug)]
pub struct ResponseWrapper {
    pub client: KintoClient,
    pub path: String,
    pub status: StatusCode,
    pub headers: Headers,
    pub json: JsonValue
}


impl Into<JsonValue> for ResponseWrapper {
    fn into(self) -> JsonValue {
        self.json
    }
}
