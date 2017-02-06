extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;

use std::io::Read;

use hyper::client;
use hyper::header::Connection;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use rustc_serialize::json;
use rustc_serialize::json::Json;


pub struct Client {
    server_url: String,
    port: u32,
    http_client: client::Client,
}


impl Client {

    // Create a client
    pub fn new(server_url:String, port:u32) -> Client {

        // Build an SSL connector
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        // Build a HTTP Client with TLS support.
        let client = client::Client::with_connector(connector);

        Client {server_url: server_url, port: port, http_client: client}
    }

    pub fn server_info(&self) -> Json {
        // Build a GET request
        let mut res = self.http_client
            .get(&self.server_url)
            .header(Connection::close())
            .send()
            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let data = Json::from_str(&body).unwrap();

        return data;
    }
}


pub fn prettyfy(data: Json) -> String {
    // Display the status
    return format!("{}", json::as_pretty_json(&data));
}
