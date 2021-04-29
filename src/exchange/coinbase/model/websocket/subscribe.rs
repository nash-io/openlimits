use serde::Deserialize;
use serde::Serialize;
use super::SubscribeCmd;
use super::Channel;
use super::Auth;

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "type")]
    pub _type: SubscribeCmd,
    pub product_ids: Vec<String>,
    pub channels: Vec<Channel>,
    #[serde(flatten)]
    pub auth: Option<Auth>,
}