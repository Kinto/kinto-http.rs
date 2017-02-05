extern crate hyper;

use std::io;
use hyper::client::Client;
use hyper::header::Connection;

fn main() {
    let client = Client::new();
    let mut res = client.get("http://www.perdu.com/")
        .header(Connection::close())
        .send().unwrap();

    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    io::copy(&mut res, &mut io::stdout()).unwrap();
}
