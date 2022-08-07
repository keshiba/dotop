use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Container {
    #[serde(rename="Id")]
    pub id:  String,
    #[serde(rename="Names")]
    pub names:  Vec<String>,
    #[serde(rename="Image")]
    pub image:  String,
    #[serde(rename="ImageID")]
    pub image_id:  String,
    #[serde(rename="Command")]
    pub command:  String,
    #[serde(rename="Created")]
    pub created:  i32,
    #[serde(rename="Ports")]
    pub ports:  Vec<Port>,
    #[serde(rename="Labels")]
    pub labels: HashMap<String, String>,
    #[serde(rename="State")]
    pub state:  String,
    #[serde(rename="Status")]
    pub status:  String,
    #[serde(rename="HostConfig")]
    pub host_config: HashMap<String, String>,
    #[serde(rename="NetworkSettings")]
    pub network_settings: HashMap<String, HashMap<String, Network>>,
    #[serde(rename="Mounts")]
    pub mounts: Vec<Mount>,
}

#[derive(Deserialize, Debug)]
pub struct Network {
    #[serde(rename="NetworkID")]
    pub network_id: String,
    #[serde(rename="EndpointID")]
    pub endpoint_id: String,
    #[serde(rename="Gateway")]
    pub gateway: String,
    #[serde(rename="IPAddress")]
    pub ip_address: String,
    #[serde(rename="IPPrefixLen")]
    pub ip_prefix_len: i32,
    #[serde(rename="IPv6Gateway")]
    pub ipv6_gateway: String,
    #[serde(rename="GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename="GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: i32,
    #[serde(rename="MacAddress")]
    pub mac_address: String,
}

#[derive(Deserialize, Debug)]
pub struct Mount {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Source")]
    pub source: String,
    #[serde(rename="Destination")]
    pub destination: String,
    #[serde(rename="Driver")]
    pub driver: String,
    #[serde(rename="Mode")]
    pub mode: String,
    #[serde(rename="RW")]
    pub rw: String,
    #[serde(rename="Propagation")]
    pub propagation: String,
}

#[derive(Deserialize, Debug)]
pub struct Port {
    #[serde(rename="PrivatePort")]
    pub private_port: i32,
    #[serde(rename="PublicPort")]
    pub public_port: i32,
    #[serde(rename="Type")]
    pub port_type: String
}

pub enum PingResponse {
    OK,
    DOWN
}