use hyper::client;
use hyper::header::{Headers, Authorization, Basic};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use error::KintoError;
use resource::Resource;
use request::KintoRequest;
use bucket::Bucket;

use utils::unwrap_collection_records;


/// Client for the Kinto HTTP API.
#[derive(Debug)]
pub struct KintoClient {
    pub server_url: String,
    pub http_client: client::Client,
    pub auth: Option<Authorization<Basic>>,
}


impl KintoClient {
    /// Create a client.
    pub fn new(server_url: String, auth: Option<Authorization<Basic>>) -> KintoClient {

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
    pub fn bucket<'a>(&self, id: &'a str) -> Bucket {
        // XXX: Cloning prevents move, but there should be a better way to
        // handle this. Using references maybe?
        Bucket::new_by_id(self.clone(), id)
    }

    /// Create a new empty bucket with a generated id.
    pub fn new_bucket(&self) -> Bucket {
        Bucket::new(self.clone())
    }

    /// List the names of all available buckets.
    pub fn list_buckets(&self) -> Result<Vec<Bucket>, KintoError> {
        let response = try!(try!(self.new_bucket().list_request()).follow_subrequests());
        return Ok(unwrap_collection_records(response, self.new_bucket()));
    }

    /// Delete all available buckets.
    pub fn delete_buckets(&self) -> Result<(), KintoError> {
        try!(try!(self.new_bucket().delete_all_request()).follow_subrequests());
        Ok(())
    }

    /// Flush the server (if the flush endpoint is enabled).
    pub fn flush(&self) -> Result<(), KintoError> {
        let path = format!("{}/__flush__", self.server_url);

        // Set authentication headers
        let mut headers = Headers::new();
        match self.auth.to_owned() {
            Some(method) => headers.set(method),
            None => (),
        };

        try!(self.http_client
                 .post(path.as_str())
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
        assert_eq!(bucket.data, None);
        assert_eq!(bucket.get_id().unwrap(), "food");
    }

    #[test]
    fn test_new_bucket() {
        let client = setup_client();
        let bucket = client.new_bucket();
        assert_eq!(bucket.data, None);
        assert_eq!(bucket.get_id(), None);
    }
}
