use reqwest::Url;
use super::json_objects::{Details, Subdomains, Tags, Whois};
use super::request;

pub fn get_details(base_url: String, api_key: String, domain: String) -> Result<Details, String> {
    let domain_url: String = format!("{}domain/{}", base_url, domain);
    let url = match Url::parse(domain_url.as_str()){
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
        let d_json: Details = match serde_json::from_str(res_text.as_str()) {
            Ok(d_json) => d_json,
            Err(e) => return Err(e.to_string()),
        };
        Ok(d_json)
    }else{
        Err(res.status().to_string())
    }
}

pub fn get_subdomains(base_url: String, api_key: String, domain: String) -> Result<Subdomains, String> {
    let subdomain_url: String = format!("{}domain/{}/subdomains", base_url, domain);
    let url = match Url::parse_with_params(subdomain_url.as_str(), &[("children_only", "false"), ("include_inactive", "true")]){
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
        let sd_json: Subdomains = match serde_json::from_str(res_text.as_str()) {
            Ok(sd_json) => sd_json,
            Err(e) => return Err(e.to_string()),
        };
        Ok(sd_json)
    }else{
        Err(res.status().to_string())
    }
}

pub fn get_tags(base_url: String, api_key: String, domain: String) -> Result<Tags, String> {
    let tags_url: String = format!("{}domain/{}/tags", base_url, domain);
    let url = match Url::parse(tags_url.as_str()){
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
        let tags_json: Tags = match serde_json::from_str(res_text.as_str()) {
            Ok(tags_json) => tags_json,
            Err(e) => return Err(e.to_string()),
        };
        Ok(tags_json)
    }else{
        Err(res.status().to_string())
    }
}

pub fn get_whois(base_url: String, api_key: String, domain: String) -> Result<Whois, String> {
    let whois_url: String = format!("{}domain/{}/whois", base_url, domain);
    let url = match Url::parse(whois_url.as_str()){
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
        let whois_json: Whois = match serde_json::from_str(res_text.as_str()) {
            Ok(tags_json) => tags_json,
            Err(e) => return Err(e.to_string()),
        };
        Ok(whois_json)
    }else{
        Err(res.status().to_string())
    }
}
