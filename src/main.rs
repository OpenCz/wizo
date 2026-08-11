use clap::Parser;

use crate::args::Commands;

mod args;
mod commands;
mod modules;
mod status;

fn main() {
    let args = args::Args::parse();

    match args.command {
        Commands::Runner { dry_run, jobs } => commands::runner::handle(dry_run, jobs),
        Commands::Check => {
            let _workflows = commands::check::workflows();
        },
    }
}
