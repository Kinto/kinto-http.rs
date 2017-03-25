extern crate hyper;
extern crate kinto_http;
#[macro_use]
extern crate serde_json;


use hyper::header::{Authorization, Basic};
use kinto_http::{KintoClient, Resource};
use serde_json::to_string_pretty;


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
    let ref mut new_record = client.bucket("default").collection("notes").new_record();

    // Set some stuff
    new_record.data = Some(json!({"title": "Hello World"}));

    // Save the record on the server or panic if fails
    new_record.set().unwrap();

    // Get the created record by id
    let mut get_record =
        client.bucket("default").collection("notes").record(new_record.id().unwrap());

    // Get the record from the server or panic if fails
    get_record.load().unwrap();

    println!("{}", to_string_pretty(get_record.data
                                              .as_ref()
                                              .unwrap()).unwrap());
}
