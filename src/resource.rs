use serde_json::Value;
use hyper::header::{IfMatch, IfNoneMatch};

use client::KintoClient;
use error::KintoError;
use request::{GetRecord, CreateRecord, UpdateRecord, DeleteRecord, KintoRequest,
              PayloadedEndpoint};
use response::ResponseWrapper;
use utils::timestamp_to_etag;


/// Implement a Kinto core resource client.
pub trait Resource: Clone {
    /// Get the path for the resource.
    fn resource_path(&self) -> Result<String, KintoError>;

    /// Get the record path for the resource.
    fn record_path(&self) -> Result<String, KintoError> {
        match self.id() {
            Some(id) => Ok(format!("{}/{}", try!(self.resource_path()), id)),
            None => Err(KintoError::UndefinedIdError),
        }
    }

    /// Unwrap a request response and update the current object.
    fn unwrap_response(&mut self, wrapper: ResponseWrapper);

    /// Get the client for the given resource.
    fn client(&self) -> KintoClient;

    /// Get the record data.
    fn data(&self) -> Option<Value>;

    /// Get the record permissions.
    fn permissions(&self) -> Option<Value>;

    /// Return the object unique id.
    fn id(&self) -> Option<&str>;

    /// Return the object version timestamp.
    fn timestamp(&self) -> Option<u64>;

    fn get_body(&self) -> Value {
        let mut body = json!({});

        // If id is defined, replace body id with the provided id
        match self.id() {
            Some(id) => {
                match self.data() {
                    Some(mut data) => {
                        data["id"] = id.into();
                        body["data"] = data;
                    }
                    None => {
                        body["data"] = json!({
                                                 "id": id
                                             })
                    }
                }
            }
            None => {
                match self.data() {
                    Some(data) => body["data"] = data,
                    None => (),
                }
            }
        };

        match self.permissions() {
            Some(perms) => {
                if perms != json!({}) {
                    body["permissions"] = perms;
                }
            }
            None => (),
        };

        return body;
    }

    /// create a custom load (GET) request for the endpoint.
    fn load_request(&self) -> Result<GetRecord, KintoError> {
        Ok(GetRecord::new(self.client(), try!(self.record_path())))
    }

    /// create a custom create (POST) request for the endpoint.
    fn create_request(&self) -> Result<CreateRecord, KintoError> {
        Ok(CreateRecord::new(self.client(), try!(self.resource_path())))
    }

    /// Create a custom update (PUT) request for the endpoint.
    fn update_request(&self) -> Result<UpdateRecord, KintoError> {
        Ok(UpdateRecord::new(self.client(), try!(self.record_path())))
    }

    /// Create a custom delete request for the endpoint.
    fn delete_request(&self) -> Result<DeleteRecord, KintoError> {
        Ok(DeleteRecord::new(self.client(), try!(self.record_path())))
    }

    /// Load bucket by id if exists.
    fn load(&mut self) -> Result<(), KintoError> {
        let wrapper = match try!(self.load_request()).send() {
            Ok(wrapper) => wrapper,
            Err(value) => return Err(value),
        };
        self.unwrap_response(wrapper);
        Ok(())
    }

    /// Set current object to the server (create or update).
    fn set(&mut self) -> Result<(), KintoError> {
        if self.id() == None {
            return self.create();
        }

        let wrapper =
            match try!(self.update_request()).body(self.get_body().into()).send() {
                Ok(wrapper) => wrapper,
                Err(value) => return Err(value),
            };
        self.unwrap_response(wrapper);
        Ok(())
    }

    /// Create if not exists the current object.
    fn create(&mut self) -> Result<(), KintoError> {
        let wrapper = match try!(self.create_request())
                  .body(self.get_body().into())
                  .if_none_match(IfNoneMatch::Any)
                  .send() {
            Ok(wrapper) => wrapper,
            Err(value) => return Err(value),
        };
        self.unwrap_response(wrapper);
        Ok(())
    }

    /// Update an existing object if exists with the current object.
    fn update(&mut self) -> Result<(), KintoError> {
        let stamp = self.timestamp();

        let if_match = match stamp {
            Some(stamp) => IfMatch::Items(timestamp_to_etag(stamp)),
            None => IfMatch::Any,
        };

        let wrapper = match self.update_request()
                  .unwrap()
                  .body(self.get_body().into())
                  .if_match(if_match)
                  .send() {
            Ok(wrapper) => wrapper,
            Err(value) => return Err(value),
        };
        self.unwrap_response(wrapper);
        Ok(())
    }

    /// Delete the current object from the server if exists.
    fn delete(&mut self) -> Result<(), KintoError> {
        let wrapper = match self.delete_request().unwrap().send() {
            Ok(wrapper) => wrapper,
            Err(value) => return Err(value),
        };
        self.unwrap_response(wrapper);
        Ok(())
    }
}
