use dotenv;
use std::env;
use std::process;

fn get_api_key() -> String {
    dotenv::dotenv().ok();
    let key = "ROUTESERVICE_API_KEY";
    match env::var(key) {
        Ok(val) => return val,
        Err(e) => {
            println!("You should define {}. Following error happened: {}", key, e);
            process::exit(1);
        }
    }
}

async fn get_geolocation(address: &str, api_key: String) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.openrouteservice.org/geocode/search?api_key={}&text={}", api_key, address);
    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;

    Ok(resp)
}

#[tokio::main]
async fn main() {
    let api_key = get_api_key();
}
