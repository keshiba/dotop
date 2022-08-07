use hyper::{Client, Uri};
use std::error::Error;
use std::net::IpAddr;
use crate::container_api::docker::model::PingResponse;
use crate::{Container, Docker};
use hyper::body::Buf;

impl Docker {

    const TRANSPORT: &'static str = "http";

    pub fn new(ip_addr: IpAddr, port: u16) -> Docker {

        let base_url = format!("{}://{}:{}", Docker::TRANSPORT, ip_addr.to_string(), port);
        return Docker {
            base_url
        };
    }

    pub async fn ping(&self)
        -> Result<PingResponse, Box<dyn Error + Send + Sync>>  {

        let ping_uri_str = format!("{}/_ping", self.base_url);
        debug!("PING: {}", ping_uri_str);

        let client = Client::new();
        let resp = client.get(ping_uri_str.parse()?).await?;

        return match resp.status().is_success() {
            true => Ok(PingResponse::OK),
            false => Ok(PingResponse::DOWN)
        }
    }

    pub async fn get_containers(&self, all: bool)
        -> Result<Vec<Container>, Box<dyn Error + Send + Sync>> {

        let return_all_param = format!("{}", all);
        let containers_uri_str = format!("{}/containers/json?all={}", self.base_url, return_all_param);
        debug!("CONTAINERS: {}", containers_uri_str);

        let client = Client::new();
        let mut resp = client.get(containers_uri_str.parse()?).await?;
        let body = hyper::body::aggregate(&mut resp).await?;
        let containers: Vec<Container> = serde_json::from_reader(body.reader())?;

        debug!("Response: {:?}", containers.iter().map(|c| &c.id));

        Ok(containers)
    }
}
