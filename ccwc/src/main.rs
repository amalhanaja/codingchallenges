mod cli;
mod counter;
mod formatter;
mod options;

use cli::parse_command;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>().join(" ");
    let output = parse_command(args).execute();
    println!("{}", output)
}
