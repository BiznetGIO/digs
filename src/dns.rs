use std::str::FromStr;

use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use trust_dns_client::udp::UdpClientConnection;

use crate::error::DigsError;

/// Record Types
#[derive(Debug, Clone, Copy)]
pub enum RTypes {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
    SOA,
    TXT,
}

impl FromStr for RTypes {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "AAAA" => Ok(Self::AAAA),
            "CNAME" => Ok(Self::CNAME),
            "MX" => Ok(Self::MX),
            "NS" => Ok(Self::NS),
            "SOA" => Ok(Self::SOA),
            "TXT" => Ok(Self::TXT),
            _ => Err("no match"),
        }
    }
}

const fn get_rtype(rtype: RTypes) -> RecordType {
    match rtype {
        RTypes::A => RecordType::A,
        RTypes::AAAA => RecordType::AAAA,
        RTypes::CNAME => RecordType::CNAME,
        RTypes::MX => RecordType::MX,
        RTypes::NS => RecordType::NS,
        RTypes::SOA => RecordType::SOA,
        RTypes::TXT => RecordType::TXT,
    }
}

/// Parse address string
fn get_address(nameserver: &str) -> Result<std::net::SocketAddr, DigsError> {
    let address = format!("{}:53", nameserver).parse::<std::net::SocketAddr>();
    match address {
        Ok(addr) => Ok(addr),
        Err(_) => Err(DigsError::InvalidIpAddress(nameserver.to_string())),
    }
}

pub fn query(domain: &str, rtype: RTypes, nameserver: &str) -> Result<DnsResponse, DigsError> {
    let address = get_address(nameserver)?;
    let conn = UdpClientConnection::new(address)?;
    let client = SyncClient::new(conn);

    let rtype = get_rtype(rtype);
    let name = Name::from_str(&format!("{}.", domain))?;
    let response = client.query(&name, DNSClass::IN, rtype);
    match response {
        Ok(resp) => Ok(resp),
        Err(err) => Err(DigsError::ForeignError(err)),
    }
}
