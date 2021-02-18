use std::str::FromStr;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::error::ClientError;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use trust_dns_client::udp::UdpClientConnection;

// Record Types
#[derive(Debug, Clone, Copy)]
pub enum RTypes {
    A,
    AAAA,
    MX,
    NS,
    SOA,
    TXT,
}

impl FromStr for RTypes {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RTypes::A),
            "AAAA" => Ok(RTypes::AAAA),
            "MX" => Ok(RTypes::MX),
            "NS" => Ok(RTypes::NS),
            "SOA" => Ok(RTypes::SOA),
            "TXT" => Ok(RTypes::TXT),
            _ => Err("no match"),
        }
    }
}

fn get_rtype(rtype: RTypes) -> RecordType {
    match rtype {
        RTypes::A => RecordType::A,
        RTypes::AAAA => RecordType::AAAA,
        RTypes::MX => RecordType::MX,
        RTypes::NS => RecordType::NS,
        RTypes::SOA => RecordType::SOA,
        RTypes::TXT => RecordType::TXT,
    }
}
// Parse address string
fn get_address(nameserver: &str) -> std::net::SocketAddr {
    let address = format!("{}:53", nameserver).parse::<std::net::SocketAddr>();
    match address {
        Err(_) => {
            eprintln!("Invalid address: '{}'", nameserver);
            std::process::exit(1);
        }
        Ok(addr) => addr,
    }
}

pub fn query(domain: &str, rtype: RTypes, nameserver: &str) -> Result<DnsResponse, ClientError> {
    let address = get_address(nameserver);
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);

    let rtype = get_rtype(rtype);
    let name = Name::from_str(&format!("{}.", domain)).unwrap();
    let response = client.query(&name, DNSClass::IN, rtype);

    return response;
}
