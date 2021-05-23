use std::fs;
use securitytrails_rs::Client;

fn main() {
    let api_key: &str = "your_api_key";
    let domain = "google.com";
    let client = Client::new(api_key).unwrap();
    let subdomains = match client.get_subdomains(domain) {
        Ok(subdomains) => subdomains,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    println!("{}", subdomains.endpoint);
    let joined = subdomains.subdomains.join("\n");
    let save_path = format!("{}_subdomains.txt", domain);
    fs::write(save_path, joined).expect("Unable to write file");
}
