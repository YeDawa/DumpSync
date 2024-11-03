use clap_cargo::style;
use clap::builder::styling;

use clap::{
    Parser,
    Subcommand,
    ColorChoice,
};

pub const CLAP_STYLING: styling::Styles = styling::Styles::styled()
    .header(style::HEADER)
    .usage(style::USAGE)
    .literal(style::LITERAL)
    .placeholder(style::PLACEHOLDER)
    .error(style::ERROR)
    .valid(style::VALID)
    .invalid(style::INVALID);

#[derive(Parser)]
#[command(styles = CLAP_STYLING)]
#[command(author, version, about, long_about = None, color = ColorChoice::Auto)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Make the database dump
    Export(ExportOptions),
    
    /// Import the database dump
    Import(ImportOptions),
}

#[derive(Parser)]
pub struct ExportOptions {
    #[arg(short, long)]
    /// Interval of the make dump (in seconds)
    pub interval: Option<u64>,

    #[arg(short, long)]
    /// Database name
    pub database: Option<String>,

    #[arg(short, long)]
    /// Backup path
    pub folder: Option<String>,
}

#[derive(Parser)]
pub struct ImportOptions {
    #[arg(short, long)]
    /// Database name
    pub database: Option<String>,

    #[arg(short, long)]
    /// Dump file path
    pub file: Option<String>,
}
