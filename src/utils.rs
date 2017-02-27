use std::collections::HashMap;

use hyper::header::EntityTag;
use json::JsonValue;

use response::ResponseWrapper;


/// Get the resource ids from a collection endpoint.
pub fn unwrap_collection_ids(wrapper: ResponseWrapper) -> Vec<String> {
    let list = wrapper.json["data"].members();
    let mut ids = vec![];
    for record in list {
        ids.push(record["id"].to_string())
    }
    return ids;
}


/// Transform an integer timestamp into an Etag header.
pub fn timestamp_to_etag(timestamp: u64) -> Vec<EntityTag> {
    let quoted = format!("{}", timestamp);
    return vec![EntityTag::new(false, quoted)];
}


/// Split a path (e.g. "/buckets/food/collections/foo") into a resource name HashMap.
pub fn extract_ids_from_path(path: String) -> HashMap<String, Option<String>> {


    // Split path into ["", "buckets", "bucket_id", ...]
    let mut split = path.split("/");

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
    println!("{:?}", map);
    return map;
}


/// Extract a list of principals from a JsonValue entry.
pub fn format_permissions(json: JsonValue) -> Vec<String> {
    let mut perms = vec![];
    for principal in json.members() {
        perms.push(principal.to_string());
    }
    return perms;
}


// pub fn follow_subrequests(preparer: RequestPreparer) -> ResponseWrapper {}


#[cfg(test)]
pub mod tests {

use hyper::header::{Authorization, Basic};

use KintoClient;
use resource::Resource;
use bucket::Bucket;
use collection::Collection;
use record::Record;


/// Create a client and clean the server.
pub fn setup_client() -> KintoClient {
    //let server_url = "https://kinto.dev.mozaws.net/v1".to_owned();
    let server_url = "http://localhost:8888/v1".to_owned();

    let auth = Authorization(
        Basic {
            username: "a".to_owned(),
            password: Some("a".to_owned()),
        }
    );
    let mut client = KintoClient::new(server_url, auth.into());
    client.flush().unwrap();
    return client;
}


pub fn setup_bucket() -> Bucket {
    let mut client = setup_client();
    return client.bucket("food");
}


pub fn setup_collection() -> Collection {
    let mut client = setup_client();
    client.bucket("food").set().unwrap();
    return client.bucket("food").collection("meat");
}


pub fn setup_record() -> Record {
    let mut client = setup_client();
    client.bucket("food").set().unwrap();
    client.bucket("food").collection("meat").set().unwrap();
    return client.bucket("food").collection("meat").record("entrecote");
}
}
