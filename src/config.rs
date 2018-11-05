use serde_derive::Deserialize;
use std::fs;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct LinkConfig {
    listener: Address,
    node: Address,
}

#[derive(Deserialize, Debug)]
struct Address {
    ip: String,
    port: u16,
}

impl Address {
    pub fn address(&self) -> SocketAddrV4 {
        SocketAddrV4::new(Ipv4Addr::from_str(&self.ip).unwrap(), self.port)
    }
}

impl LinkConfig {
    pub fn from_toml<P: AsRef<Path> + std::fmt::Display>(path: P) -> LinkConfig {
        let raw_toml = fs::read_to_string(&path).expect(&format!("{} not found!", &path));
        toml::from_str(&raw_toml).unwrap()
    }
    pub fn listener(&self) -> SocketAddrV4 {
        self.listener.address()
    }
    pub fn node(&self) -> SocketAddrV4 {
        self.node.address()
    }
    pub fn show(&self) {
        println!(
            "Node({}) <-------> Listener({})",
            self.node(),
            self.listener()
        );
    }
}
