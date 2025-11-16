use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::*,
    init::DumpSyncInit,
    addons::DumpSyncAddons,
    dumper::DumpSyncDumper,
    service::DumpSyncService,
    helpers::history::History,
};

pub struct DumpSync;

impl DumpSync {

    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        History::new().init_db()?;

        match Cli::parse().command {
            Commands::Init => DumpSyncInit.initialize().await?,
            
            Commands::Export(options) => DumpSyncDumper.export(options),
            Commands::DumpData(options) => DumpSyncDumper.export_dumpdata(options),
            Commands::Import(options) => DumpSyncDumper.import(options),
            Commands::ImportData(options) => DumpSyncDumper.import_json(options),
            Commands::Transfer(options) => DumpSyncDumper.transfer(options),
            Commands::Truncate(options) => DumpSyncDumper.truncate(options),
            
            Commands::Schema(options) => DumpSyncAddons.schema(options)?,
            Commands::Checksum(options) => DumpSyncAddons.checksum(options),
            Commands::Visual(options) => DumpSyncAddons.visual(options).await,
            Commands::Share(options) => DumpSyncAddons.share(options).await?,
            Commands::Scan(options) => DumpSyncAddons.scan_xss(options).await?,
            Commands::History { history_type, filter } => DumpSyncAddons.history(&history_type, filter),

            Commands::Pull { file } => DumpSyncService.pull(&file).await,
            Commands::Push { file } => DumpSyncService.push(&file).await,
            Commands::Login => DumpSyncService.login(),
        }

        Ok(())
    }

}
