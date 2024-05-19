use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sui-cli")]
#[command(about = "CLI tool for Sui blockchain", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        project_name: String,
    },
    Deploy {
        #[arg(short, long)]
        contract_path: String,
    },
    Test,
    Query {
        #[arg(short, long)]
        address: String,
    },
}

