
extern crate hyper;
extern crate kinto_http;
#[macro_use]
extern crate serde_json;


use hyper::header::{Authorization, Basic};
use kinto_http::{KintoClient, Resource};


fn main() {

    let server_url = "https://kinto.dev.mozaws.net/v1/";

    // Setup Hyper authentication.
    let auth = Authorization(Basic {
                                 username: "gabi".to_owned(),
                                 password: Some("my_secret".to_owned()),
                             });

    // Create a client.
    let client = KintoClient::new(server_url.to_owned(), auth.into());

    // Pick a new record using the default bucket
    let mut new_bucket = client.new_bucket();

    new_bucket.permissions.read = Some(vec!["system.Everyone".to_owned()]);
    new_bucket.data = Some(json!({"title": "Hello World"}));

    // Save the record on the server or panic if fails
    new_bucket.set().unwrap();

    println!("{:?}", new_bucket.permissions);

    // Create an unautheticated client.
    let pub_client = KintoClient::new(server_url.to_owned(), None);


    // Get the created record by id
    let mut load_bucket = pub_client.bucket(new_bucket.id().unwrap());

    // Get the record from the server or panic if fails
    load_bucket.load().unwrap();

    println!("{:?}", load_bucket.data);
}
