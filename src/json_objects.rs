use serde::{Deserialize, Serialize};
//use super::sys::bool_from_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    //#[serde(deserialize_with = "bool_from_string")]
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub current_monthly_usage: u32,
    pub allowed_monthly_usage: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct A{
    pub ip_organization: Option<String>,
    pub ip_count: u32,
    pub ip: String,
    pub h: Option<String>,
}

/* #[derive(Serialize, Deserialize, Debug)]
pub struct AAAA{
    pub ip_organization: String,
    pub ip_count: u32,
    pub ip: String,
} */

#[derive(Serialize, Deserialize, Debug)]
pub struct MX{
    pub priority: u32,
    pub hostname_organization: Option<String>,
    pub hostname_count: u32,
    pub hostname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NS{
    pub nameserver_organization: Option<String>,
    pub nameserver_count: u32,
    pub nameserver: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SOA{
    pub ttl: u32,
    pub email_count: u32,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TXT{
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsA{
    pub values: Vec<A>,
    pub first_seen: String,
}

/* #[derive(Serialize, Deserialize, Debug)]
pub struct DnsAAAA{
    pub values: Vec<AAAA>,
    pub first_seen: String,
} */

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsMX{
    pub values: Vec<MX>,
    pub first_seen: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsNS{
    pub values: Vec<NS>,
    pub first_seen: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsSOA{
    pub values: Vec<SOA>,
    pub first_seen: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsTXT {
    pub values: Vec<TXT>,
    pub first_seen: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentDns {
    pub a: DnsA,
    pub mx: DnsMX,
    pub ns: DnsNS,
    pub soa: DnsSOA,
    pub txt: DnsTXT,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    pub hostname: String,
    pub endpoint: String,
    pub current_dns: CurrentDns,
    pub alexa_rank: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subdomains {
    pub endpoint: String,
    pub subdomains: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tags {
    pub endpoint: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub city: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub name_count: Option<u32>,
    pub organization: Option<String>,
    pub organization_count: Option<u32>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub state: Option<String>,
    pub street1: Option<String>,
    pub telephone: Option<String>,
    #[serde(rename = "type")]
    pub contact_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Whois {
    #[serde(rename = "contactEmail")]
    pub contact_email: String,
    pub contacts: Vec<Contact>,
    #[serde(rename = "createdDate")]
    pub created_date: String,
    pub domain: String,
    pub endpoint: String,
    #[serde(rename = "expiresDate")]
    pub expires_date: String,
    #[serde(rename = "nameServers")]
    pub name_servers: Vec<String>,
    pub private_registration: bool,
    #[serde(rename = "registrarName")]
    pub registrar_name: String,
    pub status: String,
    #[serde(rename = "updatedDate")]
    pub updated_date: String,
}
