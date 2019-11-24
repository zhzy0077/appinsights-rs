use chrono::{DateTime, Utc};
use http::header::RETRY_AFTER;
use reqwest::blocking::Client;

use crate::contracts::Envelope;
use crate::transmitter::Transmission;
use crate::Result;

pub struct Transmitter {
    url: String,
    client: Client,
}

impl Transmitter {
    pub fn new(url: &str) -> Self {
        let client = Client::new();
        Self {
            url: url.into(),
            client,
        }
    }

    pub fn transmit(&self, items: &[Envelope]) -> Result<Transmission> {
        let payload = serde_json::to_string(items)?;

        let response = self.client.post(&self.url).body(payload).send()?;

        let status_code = response.status();

        let retry_after = if let Some(retry_after) = response.headers().get(RETRY_AFTER) {
            let retry_after = retry_after.to_str()?;
            Some(DateTime::parse_from_rfc2822(retry_after)?.with_timezone(&Utc))
        } else {
            None
        };

        Ok(Transmission::new(status_code, retry_after, response.json()?))
    }

    //    use http::StatusCode;
    //    pub fn transmit(&self, items: &[Envelope]) -> Result<Transmission> {
    //        std::thread::sleep(std::time::Duration::from_secs(1));
    //
    //        let count = items.len();
    //
    //        Ok(Transmission::new(
    //            StatusCode::OK,
    //            None,
    //            serde_json::from_value(serde_json::json!({"itemsReceived": count, "itemsAccepted": count, "errors":[] }))
    //                .unwrap(),
    //        ))
    //    }

    //    use http::StatusCode;
    //    pub fn transmit(&self, items: &[Envelope]) -> Result<Transmission> {
    //        std::thread::sleep(std::time::Duration::from_secs(1));
    //
    //        Ok(Transmission::new(
    //            StatusCode::INTERNAL_SERVER_ERROR,
    //            None,
    //            serde_json::from_value(serde_json::json!({"itemsReceived": 0, "itemsAccepted": 0, "errors":[] })).unwrap(),
    //        ))
    //    }
}