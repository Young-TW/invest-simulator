use reqwest;

fn get_yahoo() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.yahoo.com")?;
    println!("{:?}", response);
    Ok(())
}