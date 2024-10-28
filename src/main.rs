mod ui;
mod utils;
mod engine;
mod args_cli;
mod dump_sync;

use crate::dump_sync::DumpSync;

fn main() {
    DumpSync::init();
}
