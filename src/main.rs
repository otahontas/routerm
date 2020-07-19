use dotenv;
use std::env;
use std::process;
use geojson::GeoJson;
use geojson::Value::Point;

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

fn parse_geolocation(geo_result_string: &str)  -> Option<(f64, f64)> {
    if let GeoJson::FeatureCollection(parsed_geo_result) = geo_result_string.parse::<GeoJson>().unwrap() {
        if let Some(geometry) = &parsed_geo_result.features[0].geometry {
            if let Point(point)  = &geometry.value {
                return Some((point[0], point[1]));
            }
        }
    }
    None
}

#[tokio::main]
async fn main() {
    let api_key = get_api_key();
}
