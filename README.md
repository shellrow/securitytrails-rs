[crates-badge]: https://img.shields.io/crates/v/securitytrails-rs.svg
[crates-url]: https://crates.io/crates/securitytrails-rs
[license-badge]: https://img.shields.io/crates/l/securitytrails-rs.svg
[examples-url]: https://github.com/shellrow/securitytrails-rs/tree/main/examples
# securitytrails-rs [![Crates.io][crates-badge]][crates-url] ![License][license-badge]
Rust bindings for SecurityTrails API  
**Currently under development and only some features are supported**  

## Features (Unchecked items are under development)
- [x] Ping
- [x] Usage
- [x] Details
- [x] Subdomains
- [x] Tags
- [x] WHOIS
- [ ] Search
- [ ] Statistics
- [ ] Associated domains
- [ ] DNS History
- [ ] WHOIS History
- [ ] IP Neighbors
- [ ] IP Statistics


## Example
Subdomains  
```rust
use std::fs;
use securitytrails_rs::Client;

fn main() {
    let api_key: &str = "your_api_key";
    let domain = "google.com";
    let client = Client::new(api_key).unwrap();
    let ping = match client.ping() {
        Ok(ping) => ping,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    println!("ping: {}", ping.success);
    let subdomains = match client.get_subdomains(domain) {
        Ok(subdomains) => subdomains,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    println!("endpoint: {}", subdomains.endpoint);
    //println!("subdomains: {:?}", subdomains.subdomains);
    let joined = subdomains.subdomains.join("\n");
    let save_path = format!("{}_subdomains.txt", domain);
    fs::write(save_path.clone(), joined).expect("Unable to write file");
    println!("subdomains: Saved to {}", save_path);
    let usage = match client.get_usage() {
        Ok(usage) => usage,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    println!("current: {}", usage.current_monthly_usage);
    println!("allowed: {}", usage.allowed_monthly_usage);
}
```
