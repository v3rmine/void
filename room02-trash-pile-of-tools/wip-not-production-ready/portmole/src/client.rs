use std::{net::IpAddr, time::Duration};

use eyre::{Context, ContextCompat};
use futures::future;
use tokio::{
    net::{TcpStream, UdpSocket},
    time,
};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    AsyncResolver,
};

use crate::{setup_logging, ClientArgs, EyreResult, ACK_MSG, HELLO_MSG, MSG_BUFFER_LENGTH};

pub async fn handle_client(args: ClientArgs) -> EyreResult<()> {
    setup_logging(args.verbose)?;

    let resolver = AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())?;
    let resp = resolver
        .lookup_ip(args.server.clone())
        .await
        .wrap_err("Cannot find this domain name")?;
    let address = resp
        .iter()
        .next()
        .wrap_err("Cannot resolve IP from this domain name")?;

    log::debug!("Server IP resolved from {} to {}", args.server, address);

    future::join_all(
        (args.port_range.0..=args.port_range.1).map(|port| {
            spawn_tcp_udp_connection(address, port, Duration::from_millis(args.timeout))
        }),
    )
    .await
    .into_iter()
    .try_for_each(|res| {
        let res = res?;
        let res_tcp = res.0;
        let res_udp = res.1;
        EyreResult::from_iter([res_tcp, res_udp])
    })?;

    Ok(())
}

async fn spawn_tcp_udp_connection(
    address: IpAddr,
    port: u32,
    timeout: Duration,
) -> EyreResult<(EyreResult<()>, EyreResult<()>)> {
    Ok(future::join(
        spawn_tcp_connection(address, port, timeout),
        spawn_udp_connection(address, port, timeout),
    )
    .await)
}

async fn spawn_tcp_connection(address: IpAddr, port: u32, timeout: Duration) -> EyreResult<()> {
    let sever_address = format!("{}:{}", address, port);

    match time::timeout(timeout, TcpStream::connect(&sever_address)).await {
        Ok(tcp_stream) => {
            match tcp_stream {
                Ok(stream) => log::info!("TCP Connection succeed to {}", stream.peer_addr()?),
                Err(err) => {
                    log::trace!("TCP Cannot connect to {} because {}", sever_address, err);
                    log::debug!("TCP Cannot connect to {}", sever_address);
                }
            };
        }
        Err(_elapsed) => log::debug!(
            "TCP Cannot connect to {}, timeout after {}ms",
            sever_address,
            timeout.as_millis()
        ),
    }

    Ok(())
}

async fn spawn_udp_connection(address: IpAddr, port: u32, timeout: Duration) -> EyreResult<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let server_address = format!("{}:{}", address, port);

    match socket.connect(&server_address).await {
        Ok(()) => {
            let mut buf = [0; MSG_BUFFER_LENGTH];

            socket.send(HELLO_MSG).await?;
            match time::timeout(timeout, socket.recv(&mut buf)).await {
                Ok(recv_result) => {
                    let len = recv_result?;

                    if &buf[..len] == ACK_MSG {
                        log::info!("UDP Connection succeed to {}", server_address);
                    } else {
                        log::debug!("UDP Connection successful but ACK message is not good");
                    }
                }
                Err(_elapsed) => {
                    log::debug!(
                        "UDP Cannot receive from {}, timeout after {}ms",
                        server_address,
                        timeout.as_millis()
                    );
                }
            }
        }
        Err(err) => {
            log::trace!("UDP Cannot connect to {} because {}", server_address, err);
            log::debug!("UDP Cannot connect to {}", server_address);
        }
    }
    Ok(())
}
