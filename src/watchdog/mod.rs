pub mod iedred;

use crate::Location;

use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait Watchdog {
    type EqInfo;
    type Error;

    async fn watch(&mut self, tx: mpsc::Sender<()>, loction: Location) -> Result<(), Self::Error>;

    async fn scout(&self) -> Result<Self::EqInfo, Self::Error>;

    async fn bark(&self, tx: mpsc::Sender<()>) -> Result<(), Self::Error>;
}
