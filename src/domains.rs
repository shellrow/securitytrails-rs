use reqwest::header::{HeaderMap, HeaderValue,HeaderName};
use reqwest::header::ACCEPT;
use reqwest::Url;
use super::json_objects::Subdomains;

pub fn get_subdomains(base_url: String, api_key: String, domain: String) -> Result<Subdomains, String> {
    let subdomain_url: String = format!("{}domain/{}/subdomains", base_url, domain);
    let url = match Url::parse_with_params(subdomain_url.as_str(), &[("children_only", "false"), ("include_inactive", "true")]){
        Ok(url) => url,
        Err(e) => return Err(e.to_string()),
    };
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
    let res = match client.get(url).headers(headers).send() {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };
    if res.status().is_success() {
        let res_text = match res.text() {
            Ok(res_text) => res_text,
            Err(e) => return Err(e.to_string()),
        };
        let sd_json: Subdomains = match serde_json::from_str(res_text.as_str()) {
            Ok(sd_json) => sd_json,
            Err(e) => return Err(e.to_string()),
        };
        Ok(sd_json)
    }else{
        Err(res.status().to_string())
    }
}