use std::io::Read;

use serde_json;
use serde_json::Value;
use hyper::method::Method;
use hyper::header::{Headers, ContentType, IfMatch, IfNoneMatch};
use hyper::status::StatusCode;

use KintoClient;
use error::KintoError;
use response::ResponseWrapper;


/// Request builder used for setting data by specialized request methods.
#[derive(Debug, Clone)]
pub struct RequestPreparer {
    pub client: KintoClient,
    pub method: Method,
    pub path: String,
    pub headers: Headers,
    pub query: String,
    pub body: Option<Value>,
}


/// Base request data used by specialized request methods
impl RequestPreparer {
    pub fn new(client: KintoClient, path: String) -> RequestPreparer {
        RequestPreparer {
            client: client,
            method: Method::Get,
            path: path,
            headers: Headers::new(),
            query: String::new(),
            body: None,
        }
    }
}


/// Base trait with options shared with all kinto requests
pub trait KintoRequest {
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

        let full_path = format!("{}{}?{}", preparer.client.server_url,
                                           preparer.path,
                                           preparer.query);

        let mut headers = preparer.headers.to_owned();

        // Set authentication headers
        match preparer.client.auth.to_owned() {
            Some(method) => headers.set(method),
            None => (),
        };

        let payload = match preparer.body.clone() {
            Some(body) => serde_json::to_string(&body).unwrap(),
            None => "".to_owned(),
        };

        // Send prepared request
        let response = preparer.client
            .http_client
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
            client: preparer.client.to_owned(),
            path: preparer.path.to_owned(),
            status: response.status,
            headers: response.headers.to_owned(),
            body: body,
        };

        return Ok(response);
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
pub struct GetCollection {
    pub preparer: RequestPreparer,
}

impl GetCollection {
    pub fn new(client: KintoClient, path: String) -> GetCollection {
        let mut preparer = RequestPreparer::new(client, path);
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
pub struct DeleteCollection {
    pub preparer: RequestPreparer,
}

impl DeleteCollection {
    pub fn new(client: KintoClient, path: String) -> DeleteCollection {
        let mut preparer = RequestPreparer::new(client, path);
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
pub struct CreateRecord {
    pub preparer: RequestPreparer,
}

impl CreateRecord {
    pub fn new(client: KintoClient, path: String) -> CreateRecord {
        let mut preparer = RequestPreparer::new(client, path);
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
pub struct GetRecord {
    pub preparer: RequestPreparer,
}

impl GetRecord {
    pub fn new(client: KintoClient, path: String) -> GetRecord {
        let mut preparer = RequestPreparer::new(client, path);
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
pub struct UpdateRecord {
    pub preparer: RequestPreparer,
}

impl UpdateRecord {
    pub fn new(client: KintoClient, path: String) -> UpdateRecord {
        let mut preparer = RequestPreparer::new(client, path);
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
pub struct PatchRecord {
    pub preparer: RequestPreparer,
}

impl PatchRecord {
    pub fn new(client: KintoClient, path: String) -> PatchRecord {
        let mut preparer = RequestPreparer::new(client, path);
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
pub struct DeleteRecord {
    pub preparer: RequestPreparer,
}

impl DeleteRecord {
    pub fn new(client: KintoClient, path: String) -> DeleteRecord {
        let mut preparer = RequestPreparer::new(client, path);
        preparer.method = Method::Delete;
        DeleteRecord { preparer: preparer }
    }
}

impl KintoRequest for DeleteRecord {
    fn preparer(&mut self) -> &mut RequestPreparer {
        &mut self.preparer
    }
}
