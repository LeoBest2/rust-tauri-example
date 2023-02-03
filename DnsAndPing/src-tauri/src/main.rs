#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dns_lookup::lookup_host;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct IpResult {
    ip: String,
    timeout: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct CommandResult {
    name: String,
    ipv4: Vec<IpResult>,
    ipv6: Vec<IpResult>,
}

#[tauri::command]
fn query_dns_ping(addresses: Vec<&str>) -> Vec<CommandResult> {
    let mut results = Vec::new();
    for address in addresses {
        let ips = lookup_host(address).unwrap_or(Vec::new());
        let mut r = CommandResult {
            name: address.to_string(),
            ipv4: Vec::new(),
            ipv6: Vec::new(),
        };
        for ip in ips {
            let timeout = ping(&ip.to_string());
            let ip_result = IpResult {
                ip: ip.to_string(),
                timeout,
            };
            if ip.is_ipv4() {
                r.ipv4.push(ip_result);
            } else if ip.is_ipv6() {
                r.ipv6.push(ip_result);
            }
        }
        results.push(r);
    }
    results
}

#[tokio::main]
async fn ping(address: &str) -> String {
    match surge_ping::ping(address.parse().unwrap(), &[0; 32]).await {
        Ok((_packet, duration)) => format!("{:.2?}", duration),
        Err(e) => format!("{:?}", e),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![query_dns_ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
