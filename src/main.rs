extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;

use std::env;
use std::io::Read;

use hyper::client::Client;
use hyper::header::Connection;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use rustc_serialize::json;
use rustc_serialize::json::Json;


fn main() {
    // Let's get the URL from the CLI arguments
    let args: Vec<String> = env::args().collect();
    let server_url = &args[1];

    // Build an SSL connector
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);

    // Build a HTTP Client with TLS support.
    let client = Client::with_connector(connector);

    // Build a GET requests
    let mut res = client
        .get(server_url)
        .header(Connection::close())
        .send()
        .unwrap();

    // Display the status
    println!("{} {}", res.version, res.status);
    println!("{}", res.headers);

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let data = Json::from_str(&body).unwrap();


    println!("{}", json::as_pretty_json(&data));
}
