mod ui;
mod core;
mod utils;
mod plugins;
mod helpers;
mod handlers;

mod args_cli;
mod dump_sync;
mod constants;

use anyhow::Result;
use crate::dump_sync::DumpSync;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = DumpSync.init().await;
    Ok(())
}
