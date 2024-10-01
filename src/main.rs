use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.bitget.com/api/v2/spot/public/symbols";
    let response = reqwest::get(url).await?.json::<Value>().await?;

    let mut markets: Vec<String> = Vec::new();

    if let Some(data) = response["data"].as_array() {
        for symbol in data {
            if let (Some(base_coin), Some(quote_coin)) =
                (symbol["baseCoin"].as_str(), symbol["quoteCoin"].as_str())
            {
                if quote_coin == "USDT" {
                    markets.push(format!("BITGET:{}USDT", base_coin));
                }
            }
        }
    }

    markets.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split(':').collect();
        let b_parts: Vec<&str> = b.split(':').collect();
        let a_coin = a_parts[1].trim_end_matches("USDT");
        let b_coin = b_parts[1].trim_end_matches("USDT");

        let a_numeric = a_coin.chars().next().unwrap().is_numeric();
        let b_numeric = b_coin.chars().next().unwrap().is_numeric();

        match (a_numeric, b_numeric) {
            (true, true) => {
                let a_num: u32 = a_coin
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);
                let b_num: u32 = b_coin
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);
                match b_num.cmp(&a_num) {
                    std::cmp::Ordering::Equal => a_coin.cmp(b_coin),
                    other => other,
                }
            }
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (false, false) => a_coin.cmp(b_coin),
        }
    });

    let mut file = File::create("bitget_usdt_markets.txt")?;
    for market in markets {
        writeln!(file, "{}", market)?;
    }

    println!("İşlem tamamlandı. Veriler 'bitget_usdt_markets.txt' dosyasına kaydedildi.");

    Ok(())
}
