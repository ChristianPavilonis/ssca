use serde::{Deserialize, Serialize};

pub mod actions;
pub mod views;
pub mod ws;

#[derive(Deserialize)]
pub struct Person {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct HtmxMessage {
    message: String,
}

