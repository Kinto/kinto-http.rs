use std::str;
use std::io::Read;

use serde_json;
use serde_json::Value;
use hyper::method::Method;
use hyper::header::{Headers, ContentType, IfMatch, IfNoneMatch};
use hyper::status::StatusCode;

use KintoConfig;
use error::KintoError;
use response::ResponseWrapper;


/// Request builder used for setting data by specialized request methods.
#[derive(Debug, Clone)]
pub struct RequestPreparer {
    pub config: KintoConfig,
    pub method: Method,
    pub path: String,
    pub headers: Headers,
    pub query: String,
    pub body: Option<Value>,
}


/// Base request data used by specialized request methods
impl RequestPreparer {
    pub fn new(config: KintoConfig, path: String) -> RequestPreparer {
        RequestPreparer {
            config: config,
            method: Method::Get,
            path: path,
            headers: Headers::new(),
            query: String::new(),
            body: None,
        }
    }
}


/// Base trait with options shared with all kinto requests
pub trait KintoRequest: Clone {
    fn preparer(&mut self) -> &mut RequestPreparer;

    /// Set If-Match header.
    fn if_match(&mut self, if_match: IfMatch) -> &mut Self {
        self.preparer().headers.set(if_match);
        self
    }

    /// Set If-None-Match header.
    fn if_none_match(&mut self, if_match: IfNoneMatch) -> &mut Self {
        self.preparer().headers.set(if_match);
        self
    }

    /// Send the request.
    fn send(&mut self) -> Result<ResponseWrapper, KintoError> {

        // Borrow preparer mutable
        let preparer = self.preparer();

        let mut full_path = format!("{}{}",
                                preparer.config.server_url,
                                preparer.path);

        if preparer.query.len() > 0 {
            full_path = format!("{}?{}", full_path, preparer.query);
        }

        let mut headers = preparer.headers.to_owned();

        // Set authentication headers
        if let Some(ref method) = preparer.config.auth {
            headers.set(method.clone());
        }

        let payload = match preparer.body.clone() {
            Some(body) => serde_json::to_string(&body).unwrap(),
            None => "".to_owned(),
        };

        // Send prepared request
        let response = preparer
            .config
            .http_client()
            .request(preparer.method.to_owned(), &full_path)
            .headers(headers)
            .body(payload.as_str())
            .send();

        let mut response = match response {
            Ok(response) => response,
            Err(_) => return Err(KintoError::HyperError),
        };

        // Handle sync errors
        if response.status == StatusCode::NotModified {
            return Err(KintoError::NotModified);
        }

        if response.status == StatusCode::PreconditionFailed {
            return Err(KintoError::PreconditionError);
        }

        // Raise on unexpected errors
        if !response.status.is_success() {
            return Err(KintoError::HyperError);
        }

        let mut serialized = String::new();
        try!(response.read_to_string(&mut serialized));
        let body = serde_json::from_str(&serialized).unwrap();

        let response = ResponseWrapper {
            config: preparer.config.clone(),
            path: preparer.path.to_owned(),
            status: response.status,
            headers: response.headers.to_owned(),
            body: body,
        };

        return Ok(response);
    }

    fn follow_subrequests(&mut self) -> Result<ResponseWrapper, KintoError> {

        // Send first request
        let mut base_response = try!(self.send());
        let mut current_response = base_response.clone();

        loop {
            let page_header = match current_response.headers.get_raw("next-page") {
                Some(values) => values[0].clone(),
                None => return Ok(base_response),
            };

            // Gets nets page string
            let next_page_url = try!(str::from_utf8(page_header.as_slice()));

            // Repeated request on the provided endpoint
            let mut temp_request = self.clone();

            // Remove client prefix
            temp_request.preparer().path =
                next_page_url.replace(base_response.config.server_url.as_str(), "");
            temp_request.preparer().query = "".to_owned();

            current_response = try!(temp_request.send());

            // Join data fields
            let mut base_data = base_response.body["data"].as_array_mut().unwrap();
            let new_data = current_response.body["data"].as_array().unwrap();
            base_data.extend(new_data.iter().cloned());
        }
    }
}


/// Implement methods used on payloded requests (POST, PUT, PATCH).
pub trait PayloadedEndpoint: KintoRequest {
    fn body(&mut self, body: Option<Value>) -> &mut Self {
        self.preparer().headers.set(ContentType::json());
        self.preparer().body = body;
        self
    }
}

/// Implement methods used on plural endpoints (e.g. filters and pagination)
pub trait PluralEndpoint: KintoRequest {
    fn limit(&mut self, limit: i32) -> &mut Self {
        self.preparer().query = format!("{}&_limit={}", self.preparer().query, limit);
        self
    }
}

/// Get request on plural endpoints.
#[derive(Debug, Clone)]
pub struct GetCollection {
    pub preparer: RequestPreparer,
}

impl GetCollection {
    pub fn new(config: KintoConfig, path: String) -> GetCollection {
        let mut preparer = RequestPreparer::new(config, path);
        preparer.method = Method::Get;
        GetCollection { preparer: preparer }
    }
}

impl KintoRequest for GetCollection {
    fn preparer(&mut self) -> &mut RequestPreparer {
        &mut self.preparer
    }
}

impl PluralEndpoint for GetCollection {}


/// Delete request on plural endpoints.
#[derive(Debug, Clone)]
pub struct DeleteCollection {
    pub preparer: RequestPreparer,
}

impl DeleteCollection {
    pub fn new(config: KintoConfig, path: String) -> DeleteCollection {
        let mut preparer = RequestPreparer::new(config, path);
        preparer.method = Method::Delete;
        DeleteCollection { preparer: preparer }
    }
}

impl KintoRequest for DeleteCollection {
    fn preparer(&mut self) -> &mut RequestPreparer {
        &mut self.preparer
    }
}

impl PluralEndpoint for DeleteCollection {}

/// Create request on plural endpoints.
#[derive(Debug, Clone)]
pub struct CreateRecord {
    pub preparer: RequestPreparer,
}

impl CreateRecord {
    pub fn new(config: KintoConfig, path: String) -> CreateRecord {
        let mut preparer = RequestPreparer::new(config, path);
        preparer.method = Method::Post;
        CreateRecord { preparer: preparer }
    }
}

impl KintoRequest for CreateRecord {
    fn preparer(&mut self) -> &mut RequestPreparer {
        &mut self.preparer
    }
}

impl PayloadedEndpoint for CreateRecord {}


/// Get request on single endpoints.
#[derive(Debug, Clone)]
pub struct GetRecord {
    pub preparer: RequestPreparer,
}

impl GetRecord {
    pub fn new(config: KintoConfig, path: String) -> GetRecord {
        let mut preparer = RequestPreparer::new(config, path);
        preparer.method = Method::Get;
        GetRecord { preparer: preparer }
    }
}

impl KintoRequest for GetRecord {
    fn preparer(&mut self) -> &mut RequestPreparer {
        &mut self.preparer
    }
}

/// Update request on single endpoints.
#[derive(Debug, Clone)]
pub struct UpdateRecord {
    pub preparer: RequestPreparer,
}

impl UpdateRecord {
    pub fn new(config: KintoConfig, path: String) -> UpdateRecord {
        let mut preparer = RequestPreparer::new(config, path);
        preparer.method = Method::Put;
        UpdateRecord { preparer: preparer }
    }
}

impl KintoRequest for UpdateRecord {
    fn preparer(&mut self) -> &mut RequestPreparer {
        &mut self.preparer
    }
}

impl PayloadedEndpoint for UpdateRecord {}


/// Patch request on single endpoints.
#[derive(Debug, Clone)]
pub struct PatchRecord {
    pub preparer: RequestPreparer,
}

impl PatchRecord {
    pub fn new(config: KintoConfig, path: String) -> PatchRecord {
        let mut preparer = RequestPreparer::new(config, path);
        preparer.method = Method::Patch;
        PatchRecord { preparer: preparer }
    }
}

impl KintoRequest for PatchRecord {
    fn preparer(&mut self) -> &mut RequestPreparer {
        &mut self.preparer
    }
}

impl PayloadedEndpoint for PatchRecord {}


/// Delete request on single endpoints.
#[derive(Debug, Clone)]
pub struct DeleteRecord {
    pub preparer: RequestPreparer,
}

impl DeleteRecord {
    pub fn new(config: KintoConfig, path: String) -> DeleteRecord {
        let mut preparer = RequestPreparer::new(config, path);
        preparer.method = Method::Delete;
        DeleteRecord { preparer: preparer }
    }
}

impl KintoRequest for DeleteRecord {
    fn preparer(&mut self) -> &mut RequestPreparer {
        &mut self.preparer
    }
}
