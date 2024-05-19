async fn query_address(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://sui-blockchain-api/address/{}", address);

    match client.get(&url).send().await {
        Ok(res) => {
            if res.status().is_success() {
                let data: Value = res.json().await?;
                println!("Address data: {:?}", data);
            } else {
                println!("Failed to query address: {:?}", res.text().await?);
            }
        }
        Err(e) => {
            println!("Network error: {}", e);
        }
    }
    Ok(())
}
