mod ui;
mod core;
mod utils;
mod plugins;
mod helpers;
mod handlers;
mod args_cli;
mod dump_sync;
mod constants;

use std::error::Error;

use crate::dump_sync::DumpSync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = DumpSync.init().await {
        eprintln!("Error initializing app: {}", e);
        return Err(e);
    }

    Ok(())
}
