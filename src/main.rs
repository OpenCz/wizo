use clap::Parser;

use crate::{args::Commands, commands::runner};

mod args;
mod commands;

fn main() {
    let args = args::Args::parse();

    match args.command {
        Commands::Runner { dry_run, jobs } =>  {
            runner::handle(dry_run, jobs);
        },
    }
}
