pub mod json_objects;
mod request;
mod domains;
mod general;
mod sys;

use json_objects::*;

/// Base URL for SecurityTrails API V1
pub const BASE_URL_V1: &str = "https://api.securitytrails.com/v1/";

/// Client for SecurityTrails API
pub struct Client {
    /// API Key for SecurityTrails API
    api_key: String,
    /// Base URL for SecurityTrails API
    base_url: String,
}

impl Client {
    /// Construct new Client instance with API key
    pub fn new(api_key: &str) -> Result<Self, String> {
        let client = Client {
            api_key: api_key.to_string(),
            base_url: String::from(BASE_URL_V1),
        };
        Ok(client)
    }
    /// Set API key
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }
    /// Set base URL for API endpoints
    /// 
    /// Default: <https://api.securitytrails.com/v1/>
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
    /// Get API key
    pub fn get_api_key(&self) -> String {
        return self.api_key.clone();
    }
    /// Get base URL for API endpoints
    pub fn get_base_url(&self) -> String {
        return self.base_url.clone();
    }
    /// Test your authentication and access to the SecurityTrails API
    pub fn ping(&self) -> Result<Ping, String> {
        general::ping(self.base_url.clone(), self.api_key.clone())
    }
    /// Returns Usage statistics of the API for the current month
    pub fn get_usage(&self) -> Result<Usage, String> {
        general::get_usage(self.base_url.clone(), self.api_key.clone())
    }
    /// Returns the current data about the given hostname(domain)
    pub fn get_details(&self, domain: &str) -> Result<Details, String> {
        domains::get_details(self.base_url.clone(), self.api_key.clone(), domain.to_string())
    }
    /// Returns child and sibling subdomains for a given hostname(domain)
    pub fn get_subdomains(&self, domain: &str) -> Result<Subdomains, String> {
        domains::get_subdomains(self.base_url.clone(), self.api_key.clone(), domain.to_string())
    }
    /// Returns tags for a given hostname(domain)
    pub fn get_tags(&self, domain: &str) -> Result<Tags, String> {
        domains::get_tags(self.base_url.clone(), self.api_key.clone(), domain.to_string())
    }
    /// Returns the current WHOIS data about a given hostname(domain)
    pub fn get_whois(&self, domain: &str) -> Result<Whois, String> {
        domains::get_whois(self.base_url.clone(), self.api_key.clone(), domain.to_string())
    }
}
