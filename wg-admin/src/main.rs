mod actions;
mod cli;
mod config;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    match args.command {
        cli::Command::Add { name, ip, allowed_ips } => {
            actions::add_peer(&name, &ip, &allowed_ips)?;
        }
        cli::Command::Remove { name } => {
            actions::remove_peer(&name)?
        }
        cli::Command::Edit { name, ip, allowed_ips } => {
            actions::edit_peer(&name, ip.as_deref(), allowed_ips.as_deref())?
        }
        cli::Command::List => actions::list_peers()?
    }

    Ok(())
}
