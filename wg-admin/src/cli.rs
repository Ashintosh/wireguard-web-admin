use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[command(author, version, about = "WireGuard admin helper")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Add {
        name: String,
        ip: String,
        allowed_ips: String,
    },
    Remove {
        name: String,
    },
    Edit {
        name: String,
        ip: Option<String>,
        allowed_ips: Option<String>,
    },
    List,
}