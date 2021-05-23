use reqwest::header::{HeaderMap, HeaderValue,HeaderName};
use reqwest::header::ACCEPT;
use reqwest::Url;
use reqwest::blocking::Response;

pub fn get(url: Url, api_key: String) -> Result<Response, String> {
    let header_name_apikey = match HeaderName::from_lowercase(b"apikey") {
        Ok(header_name_apikey) => header_name_apikey,
        Err(e) => return Err(e.to_string()),
    };
    let header_value_apikey = match HeaderValue::from_str(api_key.as_str()) {
        Ok(header_value_apikey) => header_value_apikey,
        Err(e) => return Err(e.to_string()),
    };
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(header_name_apikey, header_value_apikey);
    let client = match reqwest::blocking::Client::builder().build() {
        Ok(client) => client,
        Err(e) => return Err(e.to_string()),
    };
    match client.get(url).headers(headers).send() {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}
