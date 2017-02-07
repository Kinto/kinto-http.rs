#[cfg(test)]
mod test {

    extern crate hyper;
    extern crate json;
    extern crate kinto_http;

    use self::hyper::header::{Authorization, Basic};
    use self::kinto_http::Client;

    static SERVER_URL: &'static str = "https://kinto.dev.mozaws.net";


    fn get_auth() -> Authorization<Basic> {
        return Authorization(
            Basic {
                username: String::from("a"),
                password: Some(String::from("a")),
            }
        );
    }

    fn get_client() -> Client {
        return Client::new(String::from(SERVER_URL), get_auth());
    }

    #[test]
    fn test_server_info() {
        let client = get_client();
        let response = client.server_info().unwrap();
        assert_eq!(response["url"], SERVER_URL.to_string()+"/v1/");
    }

    fn test_basicauth_encoding() {
        //Authorization: Basic YTph;
    }

}
