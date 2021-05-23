mod json_objects;
mod domains;

use json_objects::Subdomains;

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
    pub fn get_subdomains(&self, domain: &str) -> Result<Subdomains, String> {
        domains::get_subdomains(self.base_url.clone(), self.api_key.clone(), domain.to_string())
    }
}
