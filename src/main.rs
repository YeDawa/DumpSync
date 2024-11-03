mod ui;
mod utils;
mod engine;
mod args_cli;
mod dump_sync;

use crate::dump_sync::DumpSync;

fn main() {
    let dump_sync = DumpSync;
    dump_sync.init();
}
