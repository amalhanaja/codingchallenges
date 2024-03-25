use std::env;

use cli::parse_command;

mod cli;

fn main() {
    let args = env::args().collect::<Vec<_>>().join(" ");
    let _ = parse_command(args);
}
