use earthquaked::{
    config,
    watchdog::{self, Watchdog},
};

use eyre::Result;
use tokio::sync::mpsc;

use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Config::load("~/.config/earthquaked.yaml");
    tracing::debug!(?config, "config loaded");

    let mut watchdog = watchdog::iedred::Iedred::new();
    let (tx, mut rx) = mpsc::channel(100);
    let loc = config.location;
    tokio::spawn(async move { watchdog.watch(tx, loc).await });

    while rx.recv().await.is_some() {
        for command in &config.commands {
            if let [hd, tl @ ..] = command.split_whitespace().collect::<Vec<_>>().as_slice() {
                Command::new(hd).args(tl).spawn()?;
            }
        }
    }

    Ok(())
}
