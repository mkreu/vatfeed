use std::{borrow::Cow, fs, path::PathBuf, time::Duration};

use crate::data::*;
use reqwest::{Client, ClientBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use thiserror::Error;

pub struct Downloader {
    status: StatusStorage,
    client: Client,
}

enum StatusStorage {
    Memory(Option<Status>),
    TempFile(PathBuf),
}

static STATUS_URL: &str = "https://status.vatsim.net/status.json";

impl Downloader {
    pub fn new() -> Self {
        Self {
            status: StatusStorage::Memory(None),
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(15))
                .build()
                .unwrap(),
        }
    }

    pub fn with_status_file(path: PathBuf) -> Self {
        Self {
            status: StatusStorage::TempFile(path),
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(15))
                .build()
                .unwrap(),
        }
    }

    pub async fn download(&mut self) -> Result<Datafeed, DatafeedError> {
        let status = Self::get_or_update_status(&mut self.status, &self.client).await?;
        Self::download_json(
            &self.client,
            status.data.v3.first().ok_or(DatafeedError::NoUrlError())?,
        )
        .await
        .map_err(DatafeedError::DatafeedHttpError)
    }

    async fn get_or_update_status<'a>(
        status_storage: &'a mut StatusStorage,
        client: &Client,
    ) -> Result<Cow<'a, Status>, DatafeedError> {
        match status_storage {
            StatusStorage::Memory(status_opt) => {
                let status = match status_opt.take() {
                    Some(s) => s, //TODO check timestamp
                    None => Self::download_status(client).await?,
                };
                *status_opt = Some(status);
                Ok(Cow::Borrowed(status_opt.as_ref().unwrap()))
            }
            StatusStorage::TempFile(path) => {
                let status = match fs::read_to_string(&path)
                    .ok()
                    .and_then(|s| serde_json::from_str(&s).ok())
                {
                    Some(status) => status,
                    None => {
                        let status = Self::download_status(client).await?;
                        fs::write(&path, serde_json::to_string(&status).unwrap())?;
                        status
                    }
                };
                Ok(Cow::Owned(status))
            }
        }
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
    #[error("Failed to write status file")]
    StatusWriteError(#[from] std::io::Error),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Status {
    data: StatusData,
    user: Vec<String>,
    metar: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct StatusData {
    v3: Vec<String>,
    transceivers: Vec<String>,
}
