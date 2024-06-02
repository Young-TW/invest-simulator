use reqwest;

pub async fn get_yahoo() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.yahoo.com").await?;
    println!("{:?}", response);
    Ok(())
}