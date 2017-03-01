extern crate hyper;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate kinto_http;


use hyper::header::{Authorization, Basic};
use kinto_http::{KintoClient, Resource};


#[derive(Serialize, Deserialize)]
struct Record {
    id: String
}

/// Tutorial on how to use the client.
fn main() {

    //
    // Using the client
    //

    let server_url = "http://localhost:8888/v1";

    // Using Hyper authetication.
    let auth = Authorization(
        Basic {
            username: "Gabi".to_owned(),
            password: "MySecret".to_owned().into(),
        }
    );

    // Creating a client.
    let mut client = KintoClient::new(server_url.to_owned(),
                                      auth.into());

    let mut bucket = client.bucket("buck");
    bucket.set().unwrap();
    bucket.load().unwrap();
    println!("{:?}", bucket);

    client.flush().unwrap();
    // Creating a new bucket (safe).
    //let bucket = match client.new_bucket() {
    //    Ok(bucket) => bucket,
    //    Err(_) => panic!("Failed to create a bucket!")
    //};

    // Creating a new bucket (unsafe).
    //let mut bucket = client.new_bucket().unwrap();

    //
    // Using a bucket instance
    //

    // Getting a bucket from the client.
    //let mut bucket = client.bucket("work");

    // Creating the bucket if not exists.
    //bucket.create().unwrap();

    //bucket.update().unwrap();

    //
    //bucket.set().unwrap();
    //let get_data = client.bucket("cachaca").get_data();
    //let drinks = client.list_buckets().limit(1).send();
    //let delete = client.delete_buckets().send();

    //println!("{}", prettyfy(bucket.data.into()));
    //println!("{}", kinto_http::prettyfy(get_data.unwrap()));
    //println!("{}", kinto_http::prettyfy(drinks.unwrap()));
    //println!("{}", kinto_http::prettyfy(delete.unwrap()));
}
