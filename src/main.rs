mod ui;
mod cmd;
mod core;
mod utils;
mod cloud;
mod plugins;
mod helpers;
mod handlers;
mod constants;

mod init;
mod addons;
mod dumper;
mod service;
mod args_cli;
mod dump_sync;

use anyhow::Result;
use crate::dump_sync::DumpSync;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = DumpSync.init().await;
    Ok(())
}
