pub mod eq_info;

use crate::{
    watchdog::{iedred::eq_info::IedredEqInfo, Watchdog},
    Location, Region,
};

use chrono::Utc;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tracing::{debug, instrument};

use std::collections::HashSet;

#[derive(Debug)]
pub struct Iedred {
    processed: HashSet<IedredEqInfo>,
}

impl Iedred {
    pub fn new() -> Self {
        Iedred {
            processed: HashSet::new(),
        }
    }
}

#[derive(Error, Debug)]
pub enum IedredError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SendError(#[from] mpsc::error::SendError<()>),
}

type IedredResult<T> = Result<T, IedredError>;

#[async_trait::async_trait]
impl Watchdog for Iedred {
    type EqInfo = IedredEqInfo;
    type Error = IedredError;

    #[instrument]
    async fn watch(&mut self, tx: mpsc::Sender<()>, location: Location) -> IedredResult<()> {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;

            let info = self.scout().await?;
            debug!(?info, "info fetched");

            if !is_recent_warning(&info) {
                self.processed.insert(info);
                continue;
            }
            if self.processed.contains(&info) {
                continue;
            }
            match location {
                Location::Locale {
                    region: Region::Japan,
                    ..
                } => {
                    self.bark(tx.clone()).await?;
                }
                Location::Horizontal(_) => unimplemented!(),
            }
        }
    }

    async fn bark(&self, tx: mpsc::Sender<()>) -> IedredResult<()> {
        tx.send(()).await.map_err(IedredError::SendError)
    }

    async fn scout(&self) -> IedredResult<IedredEqInfo> {
        reqwest::get("https://api.iedred7584.com/eew/json/")
            .await
            .map_err(IedredError::ReqwestError)?
            .json::<IedredEqInfo>()
            .await
            .map_err(IedredError::ReqwestError)
    }
}

fn is_recent_warning(info: &IedredEqInfo) -> bool {
    Utc::now().timestamp() - info.announced_time() < 60
}
