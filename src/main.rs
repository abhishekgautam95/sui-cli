mod commands;
mod query_address;

use clap::Parser;
use commands::{Cli, Commands};
use std::fs;
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { project_name } => {
            println!("Initializing new Sui project: {}", project_name);
            if let Err(e) = init_project(project_name) {
                eprintln!("Error initializing project: {}", e);
            }
        }
        Commands::Deploy { contract_path } => {
            println!("Deploying contract from path: {}", contract_path);
            if let Err(e) = deploy_contract(contract_path).await {
                eprintln!("Error deploying contract: {}", e);
            }
        }
        Commands::Test => {
            println!("Running tests...");
        }
        Commands::Query { address } => {
            println!("Querying blockchain data for address: {}", address);
            if let Err(e) = query_address(address).await {
                eprintln!("Error querying address: {}", e);
            }
        }
    }
}

fn init_project(project_name: &str) -> std::io::Result<()> {
    let project_path = format!("./{}", project_name);
    fs::create_dir_all(project_path)?;
    Ok(())
}

async fn deploy_contract(contract_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contract_code = fs::read_to_string(contract_path)?;
    let client = Client::new();
    let res = client.post("https://sui-blockchain-api/deploy")
        .body(contract_code)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Contract deployed successfully!");
    } else {
        println!("Failed to deploy contract: {:?}", res.text().await?);
    }
    Ok(())
}

async fn query_address(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get(format!("https://sui-blockchain-api/address/{}", address))
        .send()
        .await?;

    if res.status().is_success() {
        let data: Value = res.json().await?;
        println!("Address data: {:?}", data);
    } else {
        println!("Failed to query address: {:?}", res.text().await?);
    }
    Ok(())
}
