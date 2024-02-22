use std::net;
use std::str::FromStr;

use hickory_client::client::{Client, SyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, RecordType};
use hickory_client::udp::UdpClientConnection;

use crate::error::Error;

pub fn query(
    domain: &str,
    rtype: RecordType,
    address: net::SocketAddr,
) -> Result<DnsResponse, Error> {
    let conn = UdpClientConnection::new(address)?;
    let client = SyncClient::new(conn);

    let name = Name::from_str(&format!("{}.", domain))?;
    Ok(client.query(&name, DNSClass::IN, rtype)?)
}
