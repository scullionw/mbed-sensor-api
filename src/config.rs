use serde_derive::Deserialize;
use std::fs;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct LinkConfig {
    listener: Address,
    node: Address,
    time_limiter: Limiter,
}

#[derive(Deserialize, Debug)]
pub struct Limiter {
    time: u64
}

#[derive(Deserialize, Debug)]
pub struct Address {
    bind: String,
    ip: String,
    port: u16,
}

impl Address {
    pub fn bind_addr(&self) -> SocketAddrV4 {
        SocketAddrV4::new(Ipv4Addr::from_str(&self.bind).unwrap(), self.port)
    }
    pub fn addr(&self) -> SocketAddrV4 {
        SocketAddrV4::new(Ipv4Addr::from_str(&self.ip).unwrap(), self.port)
    }
}

impl LinkConfig {
    pub fn from_toml<P: AsRef<Path> + std::fmt::Display>(path: P) -> LinkConfig {
        let raw_toml = fs::read_to_string(&path).unwrap_or_else(|_| panic!("{} not found!", &path));
        toml::from_str(&raw_toml).unwrap()
    }
    pub fn listener(&self) -> &Address {
        &self.listener
    }
    pub fn node(&self) -> &Address {
        &self.node
    }
    pub fn show(&self) {
        println!(
            "Node({:?}) <---/.../---> Listener({:?})",
            self.node(),
            self.listener()
        );
    }
    pub fn time(&self) -> u64 {
        self.time_limiter.time
    }
}
