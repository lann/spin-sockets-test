use std::net::SocketAddrV4;

use anyhow::Context;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use wasi::io::poll::poll;
use wasi::sockets::instance_network::instance_network;
use wasi::sockets::network::{IpAddressFamily, IpSocketAddress, Ipv4SocketAddress};
use wasi::sockets::tcp::ErrorCode;
use wasi::sockets::tcp_create_socket::create_tcp_socket;

/// A simple Spin HTTP component.
#[http_component]
fn handle_sockets_test(req: Request) -> anyhow::Result<impl IntoResponse> {
    let addr: SocketAddrV4 = spin_sdk::variables::get("address")
        .context("variables::get")?
        .parse()
        .context("parse<SocketAddrV4>")?;

    let sock = create_tcp_socket(IpAddressFamily::Ipv4).context("create_tcp_socket")?;

    eprintln!("Connecting to {addr:?}");

    sock.start_connect(
        &instance_network(),
        IpSocketAddress::Ipv4(Ipv4SocketAddress {
            address: addr.ip().octets().into(),
            port: addr.port(),
        }),
    )?;

    let (mut rx, mut tx) = match sock.finish_connect() {
        Ok(streams) => streams,
        Err(ErrorCode::WouldBlock) => {
            poll(&[&sock.subscribe()]);
            sock.finish_connect().context("finish_connect (second)")?
        }
        Err(err) => {
            return Err(err).context("finish_connect (first)");
        }
    };

    let mut msg = req.body();
    if msg.is_empty() {
        msg = b"ping";
    }
    let msg = [msg, b"\n"].concat();

    eprintln!("Sending {:?}", msg.escape_ascii().to_string());

    let n = std::io::Write::write(&mut tx, &msg)?;

    eprintln!("Wrote {n} bytes; waiting for response...");

    let mut buf = [0; 1024];
    let n = std::io::Read::read(&mut rx, &mut buf)?;

    let resp = buf[..n].trim_ascii_end().escape_ascii().to_string();

    eprintln!("Got {resp:?}");

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(resp)
        .build())
}
