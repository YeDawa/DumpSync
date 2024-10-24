use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Flags {
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
