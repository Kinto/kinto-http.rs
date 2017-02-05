extern crate hyper;

use hyper::client::Client;

fn main() {
    println!("Hello, Gabi!");
    let client = Client::new();
    let res = client.get("http://www.perdu.com/").send().unwrap();
    println!("HTTP status: {status}", status=res.status);
}
