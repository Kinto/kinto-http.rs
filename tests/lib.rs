#[cfg(test)]
mod test {
    extern crate kinto_http;
    use self::kinto_http::Client;

    static SERVER_URL: &'static str =  "https://kinto.dev.mozaws.net/v1/";

    #[test]
    fn test_server_info() {
        let client = Client::new(String::from(SERVER_URL), None);
        let data = client.server_info();
        assert_eq!(data["url"], SERVER_URL);
    }

    fn test_basicauth_encoding() {

        //Authorization: Basic YTph;
    }

}
