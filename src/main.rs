mod ui;
mod utils;
mod consts;
mod engine;
mod helpers;
mod args_cli;
mod dump_sync;

use crate::dump_sync::DumpSync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = DumpSync.init().await {
        eprintln!("Error initializing app: {}", e);
        return Err(e);
    }

    Ok(())
}
