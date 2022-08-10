#![allow(unused)]
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let body: String = reqwest::get("https://www.rust-lang.org")
            .await?
            .text()
            .await?;

        println!("body = {:?}", body);
    }

    {
        let map: HashMap<String, String> =
            reqwest::get("https://httpbin.org/ip").await?.json().await?;

        println!("{map:?}");
    }

    Ok(())
}
