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
    let details = match client.get_details(domain) {
        Ok(details) => details,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    println!("host_name: {}", details.hostname);
    println!("endpoint: {}", details.endpoint);
    println!("current_dns: {:?}", details.current_dns);
    println!("alexa_rank: {}", details.alexa_rank);
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
