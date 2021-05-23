use reqwest::Url;
use super::json_objects::{Ping, Usage};
use super::request;

pub fn ping(base_url: String, api_key: String) -> Result<Ping, String> {
    let ping_url: String = format!("{}ping", base_url);
    let url = match Url::parse(ping_url.as_str()){
        Ok(url) => url,
        Err(e) => return Err(e.to_string()),
    };
    let res = match request::get(url, api_key) {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };
    if res.status().is_success() {
        let res_text = match res.text() {
            Ok(res_text) => res_text,
            Err(e) => return Err(e.to_string()),
        };
        let ping_json: Ping = match serde_json::from_str(res_text.as_str()) {
            Ok(ping_json) => ping_json,
            Err(e) => return Err(e.to_string()),
        };
        Ok(ping_json)
    }else{
        Err(res.status().to_string())
    }
}

pub fn get_usage(base_url: String, api_key: String) -> Result<Usage, String> {
    let usage_url: String = format!("{}account/usage", base_url);
    let url = match Url::parse(usage_url.as_str()){
        Ok(url) => url,
        Err(e) => return Err(e.to_string()),
    };
    let res = match request::get(url, api_key) {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };
    if res.status().is_success() {
        let res_text = match res.text() {
            Ok(res_text) => res_text,
            Err(e) => return Err(e.to_string()),
        };
        let usage_json: Usage = match serde_json::from_str(res_text.as_str()) {
            Ok(usage_json) => usage_json,
            Err(e) => return Err(e.to_string()),
        };
        Ok(usage_json)
    }else{
        Err(res.status().to_string())
    }
}

