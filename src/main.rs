use clap::Parser;
use counter_cli::cli::Cli;

#[inline]
fn main() {
    Cli::parse().run();
}
