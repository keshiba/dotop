mod container_api;

use hyper::{Client, Uri};
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
use clap::Parser;

use container_api::docker::model::{ Container };
use container_api::docker::Docker;
use crate::container_api::docker::model::PingResponse;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[derive(Parser, Default, Debug)]
#[clap(author="imkeshiba@gmail.com", version, about="Interactive Docker CLI")]
struct Arguments {
    host: String,
    #[clap(default_value_t=2375, short, long)]
    port: u16
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let args = Arguments::parse();
    let ip_addr: IpAddr = args.host.parse()?;
    let port: u16 = args.port;
    let runtime = tokio::runtime::Runtime::new()?;
    return match runtime.block_on(async_main(ip_addr, port)) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

async fn async_main(ip_addr: IpAddr, port: u16) -> Result<(), Box<dyn Error + Send + Sync>> {

    let api = Docker::new(ip_addr, port);

    info!("Checking if docker daemon is alive");
    if let PingResponse::DOWN = api.ping().await? {
        panic!("Remote docker daemon on {}:{} is down", ip_addr, port);
    }

    info!("Docker daemon is alive");
    info!("Getting container list");
    let containers = api.get_containers(true).await?;

    containers.into_iter().for_each(|c| info!("Id: {}", c.id));

    Ok(())
}
