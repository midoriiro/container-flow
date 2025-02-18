use std::fmt::Display;

include!(concat!(env!("OUT_DIR"), "/models.rs"));

#[derive(Debug, Clone)]
pub enum PortProtocol {
    Tcp,
    Udp,
    Sctp
}

impl From<PortProtocol> for &str {
    fn from(value: PortProtocol) -> Self {
        match value {
            PortProtocol::Tcp => "tcp",
            PortProtocol::Udp => "udp",
            PortProtocol::Sctp => "sctp"
        }
    }
}

impl From<PortProtocol> for String {
    fn from(value: PortProtocol) -> Self {
        let value: &str = value.into();
        value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ExposedPort {
    port: u16,
    protocol: PortProtocol,
}

impl ExposedPort {
    pub fn new(port: u16, protocol: PortProtocol) -> Self {
        Self {
            port,
            protocol
        }
    }
}