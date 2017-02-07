extern crate hyper;
extern crate json;

use hyper::header::{Authorization, Basic};


extern crate kinto_http;


fn main() {
    // Let's get the URL from the CLI arguments
    let args: Vec<String> = std::env::args().collect();
    let server_url = &args[1];
    let auth = Authorization(
        Basic {
            username: "a".to_owned(),
            password: Some("a".to_owned()),
        }
    );
    let client = kinto_http::Client::new(server_url.clone(), auth);

    let mut record = json::JsonValue::new_object();
    record["id"] = "cachaca".into();

    //let info = client.server_info();
    let create = client.create_record("default".into(), "drinks".into(), record.into());
    let drinks = client.get_records("default".into(), "drinks".into());

    println!("{}", kinto_http::prettyfy(create.unwrap()));
    println!("{}", kinto_http::prettyfy(drinks.unwrap()));
}
