extern crate requests;
use requests::ToJson;

fn main() {
    let response = requests::get("http://httpbin.org/get").unwrap();
    assert_eq!(response.url(), "http://httpbin.org/get");
    assert_eq!(response.reason(), "OK");
    assert_eq!(response.status_code(), requests::StatusCode::Ok);

    let data = response.json().unwrap();
    assert_eq!(data["url"], "http://httpbin.org/get");
    assert_eq!(data["headers"]["Host"], "httpbin.org");
    assert_eq!(data["headers"]["User-Agent"],
               concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
}