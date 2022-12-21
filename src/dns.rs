use std::str::FromStr;

use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use trust_dns_client::udp::UdpClientConnection;

use crate::error::Error;

pub fn query(domain: &str, rtype: RecordType, nameserver: &str) -> Result<DnsResponse, Error> {
    let address = get_address(nameserver)?;
    let conn = UdpClientConnection::new(address)?;
    let client = SyncClient::new(conn);

    let name = Name::from_str(&format!("{}.", domain))?;
    let response = client.query(&name, DNSClass::IN, rtype);
    match response {
        Ok(resp) => Ok(resp),
        Err(err) => Err(Error::ForeignError(err)),
    }
}

/// Parse address string
fn get_address(nameserver: &str) -> Result<std::net::SocketAddr, Error> {
    let address = format!("{}:53", nameserver).parse::<std::net::SocketAddr>();
    match address {
        Ok(addr) => Ok(addr),
        Err(_) => Err(Error::InvalidIpAddress(nameserver.to_string())),
    }
}
