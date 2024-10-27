pub mod ui;
pub mod utils;
pub mod engine;
pub mod args_cli;
pub mod dumpsync;

use dumpsync::DumpSync;

fn main() {
    DumpSync::init();
}
