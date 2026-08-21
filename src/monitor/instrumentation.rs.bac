use std::error::Error;

use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};

use crate::monitor::streams::IoTDevice;

pub struct Instrumentation {
    client: Client,
    base_url: String,
}

impl Instrumentation {
    pub fn new(base_url: &str, token: &str) -> Result<Self, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Instrumentation {
            client,
            base_url: base_url.into(),
        })
    }

    pub async fn fetch_device_states(&self) -> Vec<IoTDevice> {
        let resp = self.client
            .get(format!("{}/api/states", self.base_url))
            .send()
            .await;
        if resp.is_err() { return Vec::new() }
        let decoded = 
            resp.unwrap().json::<Vec<HomeAssistantEntity>>()
            .await.unwrap_or(Vec::new());

        decoded
            .iter()
            .filter_map(|entity| {
                if entity.entity_id.contains("sensor.") //It should only filter for sensors
                && entity.attributes.device_class.eq(&Some("power".into())) //It should only filter sensors reporting a power class
                && entity.attributes.unit_of_measurement.eq(&Some("W".into())) //It should only filter sensors reporting watt
                && entity.attributes.friendly_name.is_some() //It should contain a friendly name
                && entity.entity_id.ne("sensor.watt_usage")
                //Demo overview full house power
                {
                    match entity.state.parse::<f64>() {
                        Ok(n) => Some(
                            (
                                entity.attributes.friendly_name.clone().unwrap(),
                                (n * 1000.0).round() as i128,
                            )
                                .into(),
                        ),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeAssistantEntity {
    pub entity_id: String,
    pub state: String,
    pub attributes: Attributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    pub friendly_name: Option<String>,
    pub unit_of_measurement: Option<String>,
    pub device_class: Option<String>,
}