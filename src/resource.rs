use serde_json;
use serde::{Serialize, Deserialize};
use hyper::header::{IfMatch, IfNoneMatch};

use error::KintoError;
use request::{GetRecord, UpdateRecord, DeleteRecord,
              KintoRequest, PayloadedEndpoint};
use response::ResponseWrapper;
use utils::timestamp_to_etag;


/// Implement a Kinto core resource client.
pub trait Resource: Serialize + Deserialize + Clone {

    /// Unwrap a request response and update the current object.
    fn unwrap_response(&mut self, wrapper: ResponseWrapper);

    /// Return the object version timestamp.
    fn get_timestamp(&mut self) -> Option<u64>;

    /// Create a custom load request for the endpoint.
    fn load_request(&mut self) -> GetRecord;

    /// Create a custom update request for the endpoint.
    fn update_request(&mut self) -> UpdateRecord;

    /// Create a custom delete request for the endpoint.
    fn delete_request(&mut self) -> DeleteRecord;

    /// Load bucket by id if exists.
    fn load(&mut self) -> Result<(), KintoError> {
        let wrapper = match self.load_request().send() {
            Ok(wrapper) => wrapper,
            Err(value) => return Err(value)
        };
        self.unwrap_response(wrapper);
        Ok(())
    }

    /// Set current object to the server (create or update).
    fn set(&mut self) -> Result<(), KintoError> {
        let wrapper = match self.update_request()
                                .body(serde_json::to_value(self.clone()).unwrap().into())
                                .send() {
            Ok(wrapper) => wrapper,
            Err(value) => return Err(value)
        };
        self.unwrap_response(wrapper);
        Ok(())
    }

    /// Create if not exists the current object.
    fn create(&mut self) -> Result<(), KintoError> {
        let wrapper = match self.update_request()
                                .body(serde_json::to_value(self.clone()).unwrap().into())
                                .if_none_match(IfNoneMatch::Any).send() {
            Ok(wrapper) => wrapper,
            Err(value) => return Err(value)
        };
        self.unwrap_response(wrapper);
        Ok(())
    }

    /// Update an existing object if exists with the current object.
    fn update(&mut self) -> Result<(), KintoError> {
        let stamp = self.get_timestamp();

        let if_match = match stamp {
            Some(stamp) => IfMatch::Items(timestamp_to_etag(stamp)),
            None => IfMatch::Any
        };

        let wrapper = match self.update_request()
                                .body(serde_json::to_value(self.clone()).unwrap().into())
                                .if_match(if_match).send() {
            Ok(wrapper) => wrapper,
            Err(value) => return Err(value)
        };
        self.unwrap_response(wrapper);
        Ok(())
    }

    /// Delete the current object from the server if exists.
    fn delete(&mut self) -> Result<(), KintoError> {
        let wrapper = match self.delete_request().send() {
            Ok(wrapper) => wrapper,
            Err(value) => return Err(value)
        };
        self.unwrap_response(wrapper);
        Ok(())
    }

}
