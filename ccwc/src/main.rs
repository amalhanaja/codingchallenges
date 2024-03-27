mod cli;
mod counter;

use cli::parse_command;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>().join(" ");
    let _ = parse_command(args);
}
