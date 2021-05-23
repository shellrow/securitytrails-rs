use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::map::Map;
//use super::sys::bool_from_string;

#[derive(Serialize, Deserialize)]
pub struct Ping {
    //#[serde(deserialize_with = "bool_from_string")]
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    pub current_monthly_usage: u32,
    pub allowed_monthly_usage: u32,
}

#[derive(Serialize, Deserialize)]
pub struct A{
    ip_organization: String,
    ip_count: u16,
    ip: String,
    h: String,
}

#[derive(Serialize, Deserialize)]
pub struct AAAA{

}

#[derive(Serialize, Deserialize)]
pub struct MX{
    priority: u16,
    hostname_organization: String,
    hostname_count: u16,
    hostname: String,
}

#[derive(Serialize, Deserialize)]
pub struct NS{
    nameserver_organization: String,
    nameserver_count: u32,
    nameserver: String,
}

#[derive(Serialize, Deserialize)]
pub struct SOA{
    ttl: u16,
    email_count: u32,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct TXT{
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct DnsA{
    values: Vec<A>,
    first_seen: String,
}

#[derive(Serialize, Deserialize)]
pub struct DnsAAAA{
    values: Vec<AAAA>,
    first_seen: String,
}

#[derive(Serialize, Deserialize)]
pub struct DnsMX{
    values: Vec<MX>,
    first_seen: String,
}

#[derive(Serialize, Deserialize)]
pub struct DnsNS{
    values: Vec<NS>,
    first_seen: String,
}

#[derive(Serialize, Deserialize)]
pub struct DnsSOA{
    values: Vec<SOA>,
    first_seen: String,
}

#[derive(Serialize, Deserialize)]
pub struct DnsTXT {
    values: Vec<TXT>,
    first_seen: String,
}

#[derive(Serialize, Deserialize)]
pub struct Details {
    pub hostname: String,
    pub endpoint: String,
    pub current_dns: Map<String, Value>,
    pub alexa_rank: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Subdomains {
    pub endpoint: String,
    pub subdomains: Vec<String>,
}
