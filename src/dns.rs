use std::str::FromStr;

use hickory_client::client::{Client, SyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, RecordType};
use hickory_client::udp::UdpClientConnection;

use crate::error::Error;

pub fn query(domain: &str, rtype: RecordType, nameserver: &str) -> Result<DnsResponse, Error> {
    let address = get_address(nameserver)?;
    let conn = UdpClientConnection::new(address)?;
    let client = SyncClient::new(conn);

    let name = Name::from_str(&format!("{}.", domain))?;
    Ok(client.query(&name, DNSClass::IN, rtype)?)
}

/// Parse address string
fn get_address(nameserver: &str) -> Result<std::net::SocketAddr, Error> {
    let address = format!("{}:53", nameserver).parse::<std::net::SocketAddr>();
    match address {
        Ok(addr) => Ok(addr),
        Err(_) => Err(Error::InvalidArgument(format!(
            "Invalid IP Address `{}`",
            &nameserver
        ))),
    }
}
