use clap::Parser;

use crate::{args::Commands, commands::runner};

mod args;
mod commands;
mod modules;
mod status;

fn main() {
    let args = args::Args::parse();

    match args.command {
        Commands::Runner { dry_run, jobs } => {
            runner::handle(dry_run, jobs);
        }
    }
}
