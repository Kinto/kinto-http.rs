extern crate kinto_http;


fn main() {
    // Let's get the URL from the CLI arguments
    let args: Vec<String> = std::env::args().collect();
    let server_url = &args[1];

    let client = kinto_http::Client::new(server_url.clone(), 80);
    let info = client.server_info();
    println!("{}", kinto_http::prettyfy(info));
}
