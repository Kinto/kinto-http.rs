use hyper::method::Method;
use hyper::header::{Headers, ContentType};
use hyper::status::StatusCode;
use serde_json::map::Map;

use KintoConfig;
use paths::Paths;
use request::{RequestPreparer, KintoRequest};
use response::ResponseWrapper;


#[derive(Debug, Clone)]
pub struct BatchRequest {
    pub preparer: RequestPreparer,
    pub requests: Vec<RequestPreparer>,
}


impl BatchRequest {
    pub fn new(config: KintoConfig) -> BatchRequest {
        let mut preparer = RequestPreparer::new(config, Paths::Batch.into());
        preparer.method = Method::Post;
        preparer.headers.set(ContentType::json());
        BatchRequest {
            preparer: preparer,
            requests: vec![],
        }
    }

    pub fn add_request<T>(&mut self, mut entry: T)
        where T: KintoRequest
    {
        self.requests.push(entry.preparer().clone());
    }
}


impl KintoRequest for BatchRequest {
    fn preparer(&mut self) -> &mut RequestPreparer {

        let mut json_requests = vec![];

        for req in self.requests.clone() {

            let mut headers = Map::new();
            for header in req.headers.iter() {
                headers.insert(header.name().to_owned(), header.to_string().into());
            }

            let entry = json!({
                "method": req.method.to_string(),
                "path": req.path,
                "body": req.body.unwrap_or(json!({})),
                "headers": headers,
            });
            json_requests.push(entry);

        }

        let body = json!({
            "requests": json_requests
        });

        self.preparer.body = body.into();

        &mut self.preparer
    }
}


#[derive(Debug, Clone)]
pub struct BatchResponseWrapper {
    pub config: KintoConfig,
    pub status: StatusCode,
    pub headers: Headers,
    pub responses: Vec<ResponseWrapper>,
}


impl From<ResponseWrapper> for BatchResponseWrapper {
    fn from(batch_wrapper: ResponseWrapper) -> Self {
        let mut responses = vec![];

        for resp in batch_wrapper.body
                .get("responses")
                .unwrap()
                .as_array()
                .unwrap() {
            let wrapper = ResponseWrapper {
                config: batch_wrapper.config.clone(),
                // XXX: Unwrap headers
                headers: Headers::new(),
                body: resp.get("body").unwrap().clone(),
                // XXX: Avoid version hardcodes
                path: resp.get("path")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .replace("/v1/", "/")
                    .to_owned(),
                status: StatusCode::Unregistered(resp.get("status")
                                                     .unwrap()
                                                     .as_u64()
                                                     .unwrap() as
                                                 u16),
            };
            responses.push(wrapper);
        }

        BatchResponseWrapper {
            config: batch_wrapper.config,
            status: batch_wrapper.status,
            headers: batch_wrapper.headers,
            responses: responses,
        }
    }
}


#[cfg(test)]
mod test_record {

    use hyper::status::StatusCode;
    use hyper::method::Method;

    use batch::{BatchRequest, BatchResponseWrapper};
    use request::KintoRequest;
    use resource::Resource;
    use utils::tests::{setup_config, setup_bucket};

    #[test]
    fn test_create_batch() {
        let config = setup_config();
        let bucket = setup_bucket();
        let mut batch = BatchRequest::new(config);
        batch.add_request(bucket.update_request().unwrap());
        let result: BatchResponseWrapper = batch.send().unwrap().into();
        assert_eq!(result.responses.len(), 1);
        assert_eq!(result.responses[0].status, StatusCode::Created);
        assert_eq!(result.responses[0].path, "/buckets/food");

    }

    #[test]
    fn test_add_batch_preserves_order() {
        let config = setup_config();
        let bucket = setup_bucket();
        let mut batch = BatchRequest::new(config);
        batch.add_request(bucket.update_request().unwrap());
        batch.add_request(bucket.delete_request().unwrap());
        let result: BatchResponseWrapper = batch.send().unwrap().into();
        assert_eq!(batch.requests[0].method, Method::Put);
        assert_eq!(batch.requests[1].method, Method::Delete);
        assert_eq!(result.responses[0].status, StatusCode::Created);
        assert_eq!(result.responses[1].status, StatusCode::Ok);
    }
}
