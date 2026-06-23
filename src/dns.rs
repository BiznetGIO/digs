use std::net;
use std::str::FromStr;

use hickory_client::client::{Client, ClientHandle};
use hickory_client::proto::rr::{DNSClass, Name, RecordType};
use hickory_client::proto::runtime::TokioRuntimeProvider;
use hickory_client::proto::udp::UdpClientStream;
use hickory_client::proto::xfer::DnsResponse;

use crate::error::Error;

pub async fn query(
    domain: &str,
    rtype: RecordType,
    address: net::SocketAddr,
) -> Result<DnsResponse, Error> {
    let stream = UdpClientStream::builder(address, TokioRuntimeProvider::new()).build();

    let (mut client, bg) = Client::connect(stream).await?;
    tokio::spawn(bg);

    let name = Name::from_str(&format!("{domain}."))?;
    Ok(client.query(name, DNSClass::IN, rtype).await?)
}
