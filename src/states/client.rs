use std::net::SocketAddr;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize, Clone, Debug)]
pub struct Client {
    pub id: String,
    pub addr: SocketAddr,
}

impl<'de> Deserialize<'de> for Client {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let id = value["id"].as_str().unwrap().to_owned();
        let addr = value["addr"].as_str().unwrap().parse().unwrap();
        Ok(Client::new(id, addr))
    }
}

impl Client {
    pub fn new(id: String, addr: SocketAddr) -> Self {
        Self { id, addr }
    }
}
