use clap_cargo::style;

use clap::{
    Parser,
    Subcommand,
    ColorChoice,
    builder::styling::Styles,
};

pub const CLAP_STYLING: Styles = Styles::styled()
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

    /// Transfer the dump to other server
    Transfer(TransferOptions),
    
    /// Initialize the new dump sync project
    Init,

    /// Scan the table for xss prevention
    Scan(ScanOptions),

    /// Share the dump or scan results
    Share(ShareOptions),

    /// Generate shcema from the database
    Schema(SchemaOptions),

    /// Generate checksum for the file
    Checksum(ChecksumOptions),
}

#[derive(Parser)]
pub struct ExportOptions {
    #[arg(short, long)]
    /// Interval of the make dump (in seconds)
    pub interval: Option<u64>,

    #[arg(short, long)]
    /// Backup path
    pub folder: Option<String>,

    #[arg(short, long)]
    /// Encryption file path
    pub encrypt: bool,

    #[arg(long)]
    /// Export the dump once then exit
    pub once: bool,
}

#[derive(Parser)]
pub struct ImportOptions {
    #[arg(short, long)]
    /// Dump file path
    pub file: Option<String>,
}

#[derive(Parser)]
pub struct TransferOptions {
    #[arg(short, long)]
    /// Dump file path
    pub file: Option<String>,
}

#[derive(Parser)]
pub struct ScanOptions {
    #[arg(short, long)]
    /// Table name for scan
    pub table: String,

    #[arg(short, long)]
    /// Payload file path
    pub payload: Option<String>,

    #[arg(short, long)]
    /// Offset for scan
    pub offset: Option<u64>,

    #[arg(short, long)]
    /// Limit for scan
    pub limit: Option<u64>,

    #[arg(short, long)]
    /// File path for output
    pub file: Option<String>,
}

#[derive(Parser)]
pub struct ShareOptions {
    #[arg(long)]
    /// Privacy level for share
    pub privacy: Option<String>,

    #[arg(short, long)]
    /// File path for share
    pub file: String,
}

#[derive(Parser)]
pub struct SchemaOptions {
    #[arg(short, long)]
    /// Output file path
    pub file: String,
}

#[derive(Parser)]
pub struct ChecksumOptions {
    #[arg(short, long)]
    /// Input file path to calculate checksum
    pub file: String,

    #[arg(short, long)]
    /// Output file path to save checksum
    pub output: Option<String>,
}