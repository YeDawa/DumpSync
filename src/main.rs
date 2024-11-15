mod ui;
mod utils;
mod consts;
mod engine;
mod helpers;
mod args_cli;
mod dump_sync;

use crate::dump_sync::DumpSync;

fn main() {
    DumpSync.init();
}
