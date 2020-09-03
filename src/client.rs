use reqwest::{redirect::Policy, Client, Proxy};
use std::time::Duration;

/// Create and return an instance of `reqwest::Client`
pub fn initialize(
    timeout: u64,
    useragent: &str,
    follow_redirects: bool,
    insecure: bool,
    proxy: Option<&str>,
) -> Client {
    // todo: integration test for this as well, specifically redirect, timeout, proxy, etc
    let policy = if follow_redirects {
        Policy::limited(10)
    } else {
        Policy::none()
    };

    let client = Client::builder()
        .timeout(Duration::new(timeout, 0))
        .user_agent(useragent)
        .danger_accept_invalid_certs(insecure)
        .redirect(policy);

    let client = if proxy.is_some() && !proxy.unwrap().is_empty() {
        match Proxy::all(proxy.unwrap()) {
            Ok(proxy_obj) => client.proxy(proxy_obj),
            Err(e) => {
                eprintln!(
                    "[!] Could not add proxy ({:?}) to Client configuration: {}",
                    proxy, e
                );
                client
            }
        }
    } else {
        // todo: do i wanna see this at the start of every run??
        eprintln!("[!] proxy ({:?}) not added to Client configuration", proxy);
        client
    };

    match client.build() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("[!] Could not create a Client with the given configuration, exiting.");
            panic!("Client::build: {}", e);
        }
    }
}
