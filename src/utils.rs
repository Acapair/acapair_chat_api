use std::{fs::File, io::Read};

use axum_server::tls_rustls::RustlsConfig;

use crate::ServerConfig;

pub fn read_server_config() -> ServerConfig {
    let mut config_file = File::open("configs/server_config.txt").unwrap();

    let mut configs_buf = String::new();
    config_file.read_to_string(&mut configs_buf).unwrap();

    let configs_uncleaned: Vec<&str> = configs_buf.split('\n').collect();

    let ip_address: Vec<&str> = configs_uncleaned[0].split(':').collect();
    let ip_address = ip_address[1].parse().unwrap();

    let port: Vec<&str> = configs_uncleaned[1].split(':').collect();
    let port = port[1].parse().unwrap();

    let max_message_counter: Vec<&str> = configs_uncleaned[2].split(':').collect();
    let max_message_counter = max_message_counter[1].parse().unwrap();

    ServerConfig {
        ip_address,
        port,
        max_message_counter,
    }
}

pub async fn tls_config() -> RustlsConfig {
    RustlsConfig::from_pem_file("certificates/fullchain.pem", "certificates/privkey.pem")
        .await
        .unwrap()
}
