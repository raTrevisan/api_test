extern crate serde;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let c = reqwest::Client::new();
    let r : SwarmResponse;
    r = c.get("http://localhost:2375/services/test_mqtt-cluster").send().await?.json().await?;
    println!("{}, {}", r.index, r.networks);
    Ok(())
   
}



#[derive(Deserialize)]
pub struct SwarmResponse{
    pub index: String,
    pub networks: String,
}