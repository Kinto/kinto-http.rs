extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::io::Read;

use hyper::client::Client;
use hyper::header::Connection;
use rustc_serialize::json;
use rustc_serialize::json::Json;


fn main() {
    // Prints each argument on a separate line
    let args: Vec<String> = env::args().collect();
    let server_url = &args[1];

    let client = Client::new();
    let mut res = client
        .get(server_url)
        .header(Connection::close())
        .send()
        .unwrap();

    println!("HTTP/1.1 {}", res.status);
    println!("{}", res.headers);

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let data = Json::from_str(&body).unwrap();


    println!("{}", json::as_pretty_json(&data));
}
