use hyper::client;
use hyper::header::{Headers, Authorization, Basic};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use error::KintoError;
use request::{KintoRequest, GetCollection, DeleteCollection, CreateRecord};
use bucket::Bucket;
use paths::Paths;
use utils::unwrap_collection_ids;


/// Client for the Kinto HTTP API.
#[derive(Debug)]
pub struct KintoClient {
    pub server_url: String,
    pub http_client: client::Client,
    pub auth: Option<Authorization<Basic>>,
}


impl KintoClient {

    /// Create a client.
    pub fn new(server_url:String, auth: Option<Authorization<Basic>>)
               -> KintoClient {

        // Build an SSL connector
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        // Build a HTTP Client with TLS support.
        let client = client::Client::with_connector(connector);

        KintoClient {
            server_url: server_url,
            http_client: client,
            auth: auth,
        }
    }

    /// Select an existing bucket.
    pub fn bucket(&self, id: &'static str) -> Bucket {
        // XXX: Cloning prevents move, but there should be a better way to
        // handle this. Using references maybe?
        Bucket::new_by_id(self.clone(), id)
    }

    /// Create a new empty bucket with a generated id.
    pub fn new_bucket(&self) -> Bucket {
        Bucket::new(self.clone())
    }

    /// List the names of all available buckets.
    pub fn list_buckets(&self) -> Result<Vec<String>, KintoError> {
        let response = try!(self.list_buckets_request().send());
        // XXX: we should follow possible subrequests
        Ok(unwrap_collection_ids(response))
    }

    /// Delete all available buckets.
    pub fn delete_buckets(&self) -> Result<(), KintoError> {
        try!(self.delete_buckets_request().send());
        Ok(())
    }

    /// Create a custom request for a new bucket.
    pub fn create_bucket_request(&self) -> CreateRecord {
        CreateRecord::new(self.clone(),  Paths::Buckets().into())
    }

    /// Create a custom request for listing buckets.
    pub fn list_buckets_request(&self) -> GetCollection {
        GetCollection::new(self.clone(), Paths::Buckets().into())
    }

    /// Create a custom request for deleting buckets.
    pub fn delete_buckets_request(&self) -> DeleteCollection {
        DeleteCollection::new(self.clone(), Paths::Buckets().into())
    }

    /// Flush the server (if the flush endpoint is enabled).
    pub fn flush(&self) -> Result<(), KintoError> {
        let path = format!("{}/__flush__", self.server_url);

        // Set authentication headers
        let mut headers = Headers::new();
        match self.auth.to_owned() {
            Some(method) => headers.set(method),
            None => ()
        };

       try!(self.http_client.post(path.as_str())
                            .headers(headers)
                            .send());
       Ok(())
    }
}


impl Clone for KintoClient {
    fn clone(&self) -> KintoClient {
        let new_client = KintoClient::new(self.server_url.to_owned(),
                                          self.auth.to_owned());
        return new_client;
    }
}


impl Default for KintoClient {
    fn default() -> KintoClient {
        let new_client = KintoClient::new("".to_owned(), None);
        return new_client;
    }
}


#[cfg(test)]
mod test_client {
    use resource::Resource;
    use utils::tests::setup_client;

    #[test]
    fn test_get_bucket() {
        let client = setup_client();
        let bucket = client.bucket("food");
        assert!(bucket.data != None);
        assert_eq!(bucket.get_id().unwrap(), "food");
    }

    #[test]
    fn test_new_bucket() {
        let client = setup_client();
        let bucket = client.new_bucket();
        assert_eq!(bucket.data, None);
        assert_eq!(bucket.get_id(), None);
    }

    #[test]
    fn test_list_buckets() {
        let client = setup_client();
        assert_eq!(client.list_buckets().unwrap().len(), 0);
        client.new_bucket().set().unwrap();
        assert_eq!(client.list_buckets().unwrap().len(), 1);
    }

    #[test]
    fn test_delete_buckets() {
        let client = setup_client();
        client.new_bucket().set().unwrap();
        assert_eq!(client.list_buckets().unwrap().len(), 1);
        client.delete_buckets().unwrap();
        assert_eq!(client.list_buckets().unwrap().len(), 0);
    }

    #[test]
    fn test_list_buckets_request() {
        let client = setup_client();
        let request = client.list_buckets_request();
        assert_eq!(request.preparer.path, "/buckets");
    }

    #[test]
    fn test_delete_buckets_request() {
        let client = setup_client();
        let request = client.delete_buckets_request();
        assert_eq!(request.preparer.path, "/buckets");
    }

    #[test]
    fn test_create_bucket_request() {
        let client = setup_client();
        let request = client.create_bucket_request();
        assert_eq!(request.preparer.path, "/buckets");
    }
}

