use std::String;
use reqwest;

fn get_coingecko() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.coingecko.com").unwrap();
    println!("{:?}", response);
    Ok(())
}

fn gecko_get_price(coin:String) -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd").unwrap();
    let price = response.text().unwrap();
    return price;
}