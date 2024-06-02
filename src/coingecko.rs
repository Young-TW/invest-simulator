use reqwest;

pub async fn get_coingecko() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.coingecko.com").await?; // Await the get function call and handle the Result properly
    println!("{:?}", response);
    Ok(())
}

pub async fn gecko_get_price(coin:String) -> Result<String, reqwest::Error> { // Change the return type to Result<String, reqwest::Error>
    let response = reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd").await?; // Await the get function call and handle the Result properly
    let price = response.text().await?; // Await the text function call and handle the Result properly
    Ok(price)
}