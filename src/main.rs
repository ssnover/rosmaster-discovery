use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use std::error::Error;
use std::net::Ipv4Addr;
use tokio::time::{timeout, Duration};

const ROSMASTER_SERVICE: &'static str = "_ros._tcp.local";

pub async fn discover_rosmaster(
    query_timeout: Duration,
) -> Result<(Ipv4Addr, u16), Box<dyn Error>> {
    timeout(query_timeout, get_rosmaster_uri()).await?
}

async fn get_rosmaster_uri() -> Result<(Ipv4Addr, u16), Box<dyn Error>> {
    let stream = mdns::discover::all(ROSMASTER_SERVICE, Duration::from_secs(5))?.listen();
    pin_mut!(stream);

    while let Some(Ok(response)) = stream.next().await {
        let addr = response.records().filter_map(self::to_ip_addr).next();
        let port = response.records().filter_map(self::to_port).next();
        if addr.is_some() && port.is_some() {
            return Ok((addr.unwrap(), port.unwrap()));
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to query",
    )))
}

fn to_ip_addr(record: &Record) -> Option<Ipv4Addr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        _ => None,
    }
}

fn to_port(record: &Record) -> Option<u16> {
    match &record.kind {
        RecordKind::SRV {
            priority: _,
            weight: _,
            port,
            target: _,
        } => Some(*port),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    match discover_rosmaster(Duration::from_secs(10)).await {
        Ok((addr, port)) => println!("{}:{}", addr, port),
        Err(_e) => eprintln!("Could not find rosmaster uri"),
    };
}
