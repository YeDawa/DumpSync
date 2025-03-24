use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::*,
    init::DumpSyncInit,
    addons::DumpSyncAddons,
    dumper::DumpSyncDumper,
};

pub struct DumpSync;

impl DumpSync {

    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        match Cli::parse().command {
            Commands::Init => DumpSyncInit.initialize().await?,
            Commands::Export(options) => DumpSyncDumper.export(options),
            Commands::Import(options) => DumpSyncDumper.import(options),
            Commands::Schema(options) => DumpSyncAddons.schema(options)?,
            Commands::Visual(options) => DumpSyncAddons.visual(options).await,
            Commands::Share(options) => DumpSyncAddons.share(options).await?,
            Commands::Scan(options) => DumpSyncAddons.scan_xss(options).await?,
            Commands::Transfer(options) => DumpSyncDumper.transfer(options),
            Commands::Checksum(options) => DumpSyncAddons.checksum(options),
            Commands::Truncate(options) => DumpSyncDumper.truncate(options),
            Commands::Pull(_options) => todo!(),
        }

        Ok(())
    }

}
