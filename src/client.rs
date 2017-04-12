use hyper::client;
use hyper::header::{Headers, Authorization, Basic};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use error::KintoError;
use resource::Resource;
use request::KintoRequest;
use bucket::Bucket;

use utils::unwrap_collection_records;

/// Configuration of the Kinto endpoint.
#[derive(Debug, Clone)]
pub struct KintoConfig {
    pub server_url: String,
    pub auth: Option<Authorization<Basic>>,
}

impl KintoConfig {
    pub fn new(server_url: String, auth: Option<Authorization<Basic>>) -> KintoConfig {
        KintoConfig {
            server_url: server_url,
            auth: auth,
        }
    }

    pub fn http_client(&self) -> client::Client {
        // Build an SSL connector
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        // Build a HTTP Client with TLS support.
        client::Client::with_connector(connector)
    }
}

/// Client for the Kinto HTTP API.
#[derive(Debug)]
pub struct KintoClient {
    pub http_client: client::Client,
    config: KintoConfig,
}


impl KintoClient {
    /// Create a client.
    pub fn new(config: KintoConfig) -> KintoClient {

        KintoClient {
            config: config.clone(),
            http_client: config.http_client(),
        }
    }

    /// Select an existing bucket.
    pub fn bucket(&self, id: &str) -> Bucket {
        // XXX: Cloning prevents move, but there should be a better way to
        // handle this. Using references maybe?
        Bucket::new_by_id(self.config.clone(), id)
    }

    /// Create a new empty bucket with a generated id.
    pub fn new_bucket(&self) -> Bucket {
        Bucket::new(self.config.clone())
    }

    /// List the names of all available buckets.
    pub fn list_buckets(&self) -> Result<Vec<Bucket>, KintoError> {
        let response = try!(try!(self.new_bucket().list_request()).follow_subrequests());
        Ok(unwrap_collection_records(&response, &self.new_bucket()))
    }

    /// Delete all available buckets.
    pub fn delete_buckets(&self) -> Result<(), KintoError> {
        try!(try!(self.new_bucket().delete_all_request()).follow_subrequests());
        Ok(())
    }

    /// Flush the server (if the flush endpoint is enabled).
    pub fn flush(&self) -> Result<(), KintoError> {
        // Set authentication headers
        let mut headers = Headers::new();
        if let Some(ref method) = self.config.auth {
            headers.set(method.clone());
        }

        try!(self.http_client
                 .post(&format!("{}/__flush__", self.config.server_url))
                 .headers(headers)
                 .send());
        Ok(())
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
