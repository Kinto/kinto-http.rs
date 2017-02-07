extern crate hyper;
extern crate hyper_native_tls;
extern crate json;
extern crate rustc_serialize;

use std::io::Read;

use hyper::client;
use hyper::method::Method;
use hyper::header::{Headers, Authorization, Basic, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::Error as HyperError;
use json::Error as JsonError;
use json::JsonValue;


#[derive(Debug)]
pub enum KintoError {
    HyperError,
    JsonError,
}

impl KintoError {
    pub fn from_call<T>(result: Result<T, KintoError>) -> Result<T, KintoError> {
        match result {
            Ok(result) => Ok(result),
            Err(KintoError::HyperError) => Err(KintoError::HyperError),
            Err(KintoError::JsonError) => Err(KintoError::JsonError),
        }
    }
}


pub struct Client {
    server_url: String,
    http_client: client::Client,
    headers: Headers,
}


impl Client {

    // Create a client

    pub fn new(server_url:String, auth: Authorization<Basic>) -> Client {
        // Build an SSL connector
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        // Build a HTTP Client with TLS support.
        let client = client::Client::with_connector(connector);

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        headers.set(auth.to_owned());

        let url = format!("{}/{}", server_url, "v1");

        Client {
            server_url: url,
            headers: headers,
            http_client: client,
        }
    }

    // Utilities

    pub fn server_info(&self) -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/", self.server_url);
        return self.handle_request(Method::Get, endpoint, None);
    }

    // Buckets

    pub fn get_buckets(&self) -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets", self.server_url);
        return self.handle_request(Method::Get, endpoint, None);
    }

    pub fn delete_buckets(&self) -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets", self.server_url);
        return self.handle_request(Method::Delete, endpoint, None);
    }

    pub fn get_bucket(&self, bucket_id: String) -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}", self.server_url,
                               bucket_id=bucket_id);
        return self.handle_request(Method::Get, endpoint, None);
    }

    pub fn delete_bucket(&self, bucket_id: String) -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}", self.server_url,
                               bucket_id=bucket_id);
        return self.handle_request(Method::Delete, endpoint, None);
    }

    pub fn create_bucket(&self, data: Option<JsonValue>)
                         -> Result<JsonValue, KintoError> {
        return self.create_bucket_with_perms(data, None);
    }

    pub fn create_bucket_with_perms(&self,
                                    data: Option<JsonValue>,
                                    permissions: Option<JsonValue>)
                                    -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets", self.server_url);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }

    pub fn update_bucket(&self, bucket_id: String, data: Option<JsonValue>)
                         -> Result<JsonValue, KintoError> {
        return self.update_bucket_with_perms(bucket_id, data, None);
    }

    pub fn update_bucket_with_perms(&self, bucket_id: String,
                                    data: Option<JsonValue>,
                                    permissions: Option<JsonValue>)
                                    -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}", self.server_url,
                               bucket_id=bucket_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Put, endpoint, payload.into());
    }

    pub fn patch_bucket(&self, bucket_id: String, data: Option<JsonValue>)
                        -> Result<JsonValue, KintoError> {
        return self.patch_bucket_with_perms(bucket_id, data, None);
    }

    pub fn patch_bucket_with_perms(&self, bucket_id: String,
                                   data: Option<JsonValue>,
                                   permissions: Option<JsonValue>)
                                   -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}", self.server_url,
                               bucket_id=bucket_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Patch, endpoint, payload.into());
    }

   // Groups

    pub fn get_groups(&self, bucket_id: String)
                      -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/groups",
                               self.server_url, bucket_id=bucket_id);
        return self.handle_request(Method::Get, endpoint, None);
    }

    pub fn delete_groups(&self, bucket_id: String)
                         -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/groups",
                               self.server_url, bucket_id=bucket_id);
        return self.handle_request(Method::Delete, endpoint, None);
    }

    pub fn get_group(&self, bucket_id: String, group_id: String)
                     -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/groups/{group_id}",
                               self.server_url, bucket_id=bucket_id, group_id=group_id);
        return self.handle_request(Method::Get, endpoint, None);
    }

    pub fn delete_group(&self, bucket_id: String, group_id: String)
                        -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/groups/{group_id}",
                               self.server_url, bucket_id=bucket_id, group_id=group_id);
        return self.handle_request(Method::Delete, endpoint, None);
    }

    pub fn create_group(&self, bucket_id: String, data: Option<JsonValue>)
                        -> Result<JsonValue, KintoError> {
        return self.create_group_with_perms(bucket_id, data, None);
    }

    pub fn create_group_with_perms(&self, bucket_id: String,
                                    data: Option<JsonValue>,
                                    permissions: Option<JsonValue>)
                                    -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}/groups",
                               self.server_url, bucket_id=bucket_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }

    pub fn update_group(&self, bucket_id: String, group_id: String, data: Option<JsonValue>)
                         -> Result<JsonValue, KintoError> {
        return self.update_group_with_perms(bucket_id, group_id, data, None);
    }

    pub fn update_group_with_perms(&self, bucket_id: String, group_id: String,
                                    data: Option<JsonValue>,
                                    permissions: Option<JsonValue>)
                                    -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}/groups/{group_id}",
                               self.server_url, bucket_id=bucket_id, group_id=group_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }

    pub fn patch_group(&self, bucket_id: String, group_id: String, data: Option<JsonValue>)
                        -> Result<JsonValue, KintoError> {
        return self.patch_group_with_perms(bucket_id, group_id, data, None);
    }

    pub fn patch_group_with_perms(&self, bucket_id: String, group_id: String,
                                   data: Option<JsonValue>,
                                   permissions: Option<JsonValue>)
                                   -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}/groups/{group_id}",
                               self.server_url, bucket_id=bucket_id, group_id=group_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }


    pub fn set_payload(&self, data: Option<JsonValue>,
                       permissions: Option<JsonValue>) -> JsonValue {

        let mut payload = JsonValue::new_object();

        payload["data"] = data.unwrap_or(JsonValue::new_object());
        payload["permissions"] = permissions.unwrap_or(JsonValue::new_object());

        return payload;
    }

    pub fn handle_request(&self, method: Method, endpoint: String,
                          payload: Option<JsonValue>)
                          -> Result<JsonValue, KintoError> {

        let request = self.http_client
            .request(method, &endpoint)
            .headers(self.headers.to_owned())
            .body(payload.unwrap_or(JsonValue::new_object()).dump().as_str())
            .send();

        let mut response = match request {
            Ok(response) => response,
            Err(_) => return Err(KintoError::HyperError),
        };

        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();
        let data = json::parse(&body);

        match data {
            Ok(response) => return Ok(response),
            Err(_) => return Err(KintoError::JsonError),
        };
    }

   // Collections

    pub fn get_collections(&self, bucket_id: String)
                      -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/collections",
                               self.server_url, bucket_id=bucket_id);
        return self.handle_request(Method::Get, endpoint, None);
    }

    pub fn delete_collections(&self, bucket_id: String)
                         -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/collections",
                               self.server_url, bucket_id=bucket_id);
        return self.handle_request(Method::Delete, endpoint, None);
    }

    pub fn get_collection(&self, bucket_id: String, collection_id: String)
                     -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id);
        return self.handle_request(Method::Get, endpoint, None);
    }

    pub fn delete_collection(&self, bucket_id: String, collection_id: String)
                             -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id);
        return self.handle_request(Method::Delete, endpoint, None);
    }

    pub fn create_collection(&self, bucket_id: String, data: Option<JsonValue>)
                             -> Result<JsonValue, KintoError> {
        return self.create_collection_with_perms(bucket_id, data, None);
    }

    pub fn create_collection_with_perms(&self, bucket_id: String,
                                        data: Option<JsonValue>,
                                        permissions: Option<JsonValue>)
                                        -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}/collections",
                               self.server_url, bucket_id=bucket_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }

    pub fn update_collection(&self, bucket_id: String, collection_id: String,
                             data: Option<JsonValue>) -> Result<JsonValue, KintoError> {
        return self.update_collection_with_perms(bucket_id, collection_id, data, None);
    }

    pub fn update_collection_with_perms(&self, bucket_id: String, collection_id: String,
                                        data: Option<JsonValue>,
                                        permissions: Option<JsonValue>)
                                        -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }

    pub fn patch_collection(&self, bucket_id: String, collection_id: String,
                            data: Option<JsonValue>) -> Result<JsonValue, KintoError> {
        return self.patch_collection_with_perms(bucket_id, collection_id, data, None);
    }

    pub fn patch_collection_with_perms(&self, bucket_id: String, collection_id: String,
                                       data: Option<JsonValue>,
                                       permissions: Option<JsonValue>)
                                       -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }


   // Records


    pub fn get_records(&self, bucket_id: String, collection_id: String)
                       -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}/records",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id);
        return self.handle_request(Method::Get, endpoint, None);
    }

    pub fn delete_records(&self, bucket_id: String, collection_id: String)
                         -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}/records",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id);
        return self.handle_request(Method::Delete, endpoint, None);
    }

    pub fn get_record(&self, bucket_id: String, collection_id: String, record_id: String)
                     -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}/records/{id}",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id, id=record_id);
        return self.handle_request(Method::Get, endpoint, None);
    }

    pub fn delete_record(&self, bucket_id: String, collection_id: String, record_id: String)
                        -> Result<JsonValue, KintoError> {
        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}/records/{id}",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id, id=record_id);
        return self.handle_request(Method::Delete, endpoint, None);
    }

    pub fn create_record(&self, bucket_id: String, collection_id: String,
                         data: Option<JsonValue>) -> Result<JsonValue, KintoError> {
        return self.create_record_with_perms(bucket_id, collection_id, data, None);
    }

    pub fn create_record_with_perms(&self, bucket_id: String, collection_id: String,
                                    data: Option<JsonValue>,
                                    permissions: Option<JsonValue>)
                                    -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}/records",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }

    pub fn update_record(&self, bucket_id: String, record_id: String, collection_id: String,
                         data: Option<JsonValue>) -> Result<JsonValue, KintoError> {
        return self.update_record_with_perms(bucket_id, collection_id, record_id, data, None);
    }

    pub fn update_record_with_perms(&self, bucket_id: String, collection_id: String,
                                    record_id: String, data: Option<JsonValue>,
                                    permissions: Option<JsonValue>)
                                    -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}/records/{id}",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id, id=record_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }

    pub fn patch_record(&self, bucket_id: String, collection_id: String,
                        record_id: String, data: Option<JsonValue>)
                        -> Result<JsonValue, KintoError> {
        return self.patch_record_with_perms(bucket_id, collection_id, record_id, data, None);
    }

    pub fn patch_record_with_perms(&self, bucket_id: String, collection_id: String,
                                   record_id: String, data: Option<JsonValue>,
                                   permissions: Option<JsonValue>)
                                   -> Result<JsonValue, KintoError> {

        let endpoint = format!("{}/buckets/{bucket_id}/collections/{collection_id}/records/{id}",
                               self.server_url, bucket_id=bucket_id,
                               collection_id=collection_id, id=record_id);
        let payload = self.set_payload(data, permissions);
        return self.handle_request(Method::Post, endpoint, payload.into());
    }

}


pub fn prettyfy(data: JsonValue) -> String {
    // Display the status
    return json::stringify_pretty(data, 4);
}
