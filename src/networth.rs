fn calculate_networth() {
    // Path: src/networth.rs
    // Compare this snippet from src/yahoo.rs:
    // use reqwest;
    // 
    // fn get_yahoo() -> Result<(), reqwest::Error> {
    //     let response = reqwest::get("https://www.yahoo.com")?;
    //     println!("{:?}", response);
    //     Ok(())
    // }
    // Compare this snippet from src/main.rs:
    // use reqwest;
    // 
    // fn main() {
    //     
    // }
    let response = reqwest::get("https://www.yahoo.com").unwrap();
    println!("{:?}", response);
}