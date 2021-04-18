use std::time::Duration;

use crate::data::*;
use reqwest::{Client, ClientBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use thiserror::Error;

pub struct Downloader {
    status: Option<Status>,
    client: Client,
}

static STATUS_URL: &str = "https://status.vatsim.net/status.json";

impl Downloader {
    pub fn init() -> Self {
        Self {
            status: None,
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(15))
                .build()
                .unwrap(),
        }
    }

    pub async fn download(&mut self) -> Result<Datafeed, DatafeedError> {
        let status = Self::get_or_update_status(&mut self.status, &self.client).await?;
        Self::download_json(&self.client, status.data.v3.first().ok_or(DatafeedError::NoUrlError())?)
            .await
            .map_err(DatafeedError::DatafeedHttpError)
    }

    async fn get_or_update_status<'a>(
        status_opt: &'a mut Option<Status>,
        client: &Client,
    ) -> Result<&'a Status, DatafeedError> {
        let status = match status_opt.take() {
            Some(s) => s, //TODO check timestamp
            None => Self::download_status(client).await?,
        };
        *status_opt = Some(status);
        Ok(status_opt.as_ref().unwrap())
    }
    async fn download_status(client: &Client) -> Result<Status, DatafeedError> {
        Self::download_json(client, STATUS_URL)
            .await
            .map_err(DatafeedError::StatusHttpError)
    }

    async fn download_json<T>(client: &Client, url: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        Ok(client.get(url).send().await?.json().await?)
    }
}

#[derive(Error, Debug)]
pub enum DatafeedError {
    #[error("The status file does not contain any url")]
    NoUrlError(),
    #[error("Failed to download status file")]
    StatusHttpError(reqwest::Error),
    #[error("Failed to download datafeed")]
    DatafeedHttpError(reqwest::Error),
}

#[derive(Debug, Deserialize, Serialize)]
struct Status {
    data: StatusData,
    user: Vec<String>,
    metar: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct StatusData {
    v3: Vec<String>,
    transceivers: Vec<String>,
}
