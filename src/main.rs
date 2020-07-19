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

#[tokio::main]
async fn main() {


}
