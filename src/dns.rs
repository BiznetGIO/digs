use std::str::FromStr;

use hickory_client::client::{Client, SyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, RecordType};
use hickory_client::udp::UdpClientConnection;

use crate::error::Error;

pub fn query(domain: &str, rtype: RecordType, address: &str) -> Result<DnsResponse, Error> {
    let socket_address = parse_address(address)?;
    let conn = UdpClientConnection::new(socket_address)?;
    let client = SyncClient::new(conn);

    let name = Name::from_str(&format!("{}.", domain))?;
    Ok(client.query(&name, DNSClass::IN, rtype)?)
}

/// Parse address string into `SocketAddr`
fn parse_address(address: &str) -> Result<std::net::SocketAddr, Error> {
    if address.contains(':') {
        Ok(address.parse::<std::net::SocketAddr>()?)
    } else {
        // Use default port
        Ok(format!("{address}:53").parse::<std::net::SocketAddr>()?)
    }
}
