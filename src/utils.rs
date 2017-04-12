use std::collections::HashMap;

use hyper::header::EntityTag;

use response::ResponseWrapper;
use resource::Resource;


/// Get the obkects from a plural endpoint.
pub fn unwrap_collection_records<T>(wrapper: &ResponseWrapper, object: &T) -> Vec<T>
    where T: Resource
{
    let mut records = vec![];
    for obj in wrapper.body["data"].as_array().unwrap() {
        let record = object.clone().set_data(obj.clone());
        records.push(record);
    }
    records
}


/// Transform an integer timestamp into an Etag header.
pub fn timestamp_to_etag(timestamp: u64) -> Vec<EntityTag> {
    let quoted = format!("{}", timestamp);
    vec![EntityTag::new(false, quoted)]
}


/// Split a path (e.g. "/buckets/food/collections/foo") into a resource name `HashMap`.
pub fn extract_ids_from_path(path: &str) -> HashMap<String, Option<String>> {

    // XXX: Remove version from path if exists. We shouldn't hardcode version
    let path = path.replace("/v1", "").to_owned();

    // Split path into ["", "buckets", "bucket_id", ...]
    let mut split = path.split('/');

    // Remove starting "/"
    split.next().unwrap();

    let mut map = HashMap::new();

    while let Some(key) = split.next() {
        let value = match split.next() {
            Some(v) => Some(v.to_owned()),
            None => None,
        };
        map.insert(key.to_owned(), value);
    }
    map
}


#[cfg(test)]
pub mod tests {

    use hyper::header::{Authorization, Basic};

    use KintoClient;
    use KintoConfig;
    use resource::Resource;
    use bucket::Bucket;
    use collection::Collection;
    use record::Record;

    /// Create a config.
    pub fn setup_config() -> KintoConfig {
        //let server_url = "https://kinto.dev.mozaws.net/v1".to_owned();
        let server_url = "http://localhost:8888/v1".to_owned();

        let auth = Authorization(Basic {
                                     username: "a".to_owned(),
                                     password: Some("a".to_owned()),
                                 });
        KintoConfig::new(server_url, auth.into())
    }

    /// Create a client and clean the server.
    pub fn setup_client() -> KintoClient {
        let client = KintoClient::new(setup_config());
        client.flush().unwrap();
        client
    }


    pub fn setup_bucket() -> Bucket {
        let client = setup_client();
        return client.bucket("food");
    }


    pub fn setup_collection() -> Collection {
        let client = setup_client();
        client.bucket("food").set().unwrap();
        return client.bucket("food").collection("meat");
    }


    pub fn setup_record() -> Record {
        let client = setup_client();
        client.bucket("food").set().unwrap();
        client.bucket("food").collection("meat").set().unwrap();
        return client
                   .bucket("food")
                   .collection("meat")
                   .record("entrecote");
    }
}
