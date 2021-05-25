pub mod json_objects;
mod request;
mod domains;
mod general;
mod sys;

use json_objects::*;

pub const BASE_URL_V1: &str = "https://api.securitytrails.com/v1/";

pub struct Client {
    api_key: String,
    base_url: String,
}

impl Client {
    pub fn new(api_key: &str) -> Result<Self, String> {
        let client = Client {
            api_key: api_key.to_string(),
            base_url: String::from(BASE_URL_V1),
        };
        Ok(client)
    }
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
    pub fn get_api_key(&self) -> String {
        return self.api_key.clone();
    }
    pub fn get_base_url(&self) -> String {
        return self.base_url.clone();
    }
    pub fn ping(&self) -> Result<Ping, String> {
        general::ping(self.base_url.clone(), self.api_key.clone())
    }
    pub fn get_usage(&self) -> Result<Usage, String> {
        general::get_usage(self.base_url.clone(), self.api_key.clone())
    }
    pub fn get_details(&self, domain: &str) -> Result<Details, String> {
        domains::get_details(self.base_url.clone(), self.api_key.clone(), domain.to_string())
    }
    pub fn get_subdomains(&self, domain: &str) -> Result<Subdomains, String> {
        domains::get_subdomains(self.base_url.clone(), self.api_key.clone(), domain.to_string())
    }
}
