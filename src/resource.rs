use serde_json;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use hyper::header::{IfMatch, IfNoneMatch};

use error::KintoError;
use request::{GetRecord, CreateRecord, UpdateRecord, DeleteRecord,
              KintoRequest, PayloadedEndpoint};
use response::ResponseWrapper;
use utils::timestamp_to_etag;


/// Implement a Kinto core resource client.
pub trait Resource: Serialize + Deserialize + Clone {

    /// Unwrap a request response and update the current object.
    fn unwrap_response(&mut self, wrapper: ResponseWrapper);

    fn get_data(&self) -> Option<&Value>;

    /// Return the object unique id.
    fn get_id(&self) -> Option<&str> {
        match self.get_data() {
            Some(data) => match data["id"].as_str() {
                Some(id) => id.into(),
                None => None
            },
            None => None
        }
    }

    /// Return the object version timestamp.
    fn get_timestamp(&self) -> Option<u64> {
        match self.get_data() {
            Some(data) => match data["lat_modified"].as_u64() {
                Some(ts) => ts.into(),
                None => None
            },
            None => None
        }
    }

    /// create a custom load (GET) request for the endpoint.
    fn load_request(&self) -> GetRecord;

    /// create a custom create (POST) request for the endpoint.
    fn create_request(&self) -> CreateRecord;

    /// Create a custom update (PUT) request for the endpoint.
    fn update_request(&self) -> UpdateRecord;

    /// Create a custom delete request for the endpoint.
    fn delete_request(&self) -> DeleteRecord;

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
        if self.get_id() == None {
            return self.create();
        }

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
        let wrapper = match self.create_request()
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
