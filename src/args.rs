use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Runner {
        #[arg(long)]
        dry_run: bool,

        #[arg(short, long, default_value_t = 1)]
        jobs: u32,
    },
}
