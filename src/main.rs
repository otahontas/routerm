use std::env;
use std::process;
use geojson::GeoJson;
use geojson::Value::Point;

fn get_api_key() -> String {
    dotenv::dotenv().ok();
    let key = "ROUTESERVICE_API_KEY";
    match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            println!("You should define {}. Following error happened: {}", key, e);
            process::exit(1);
        }
    }
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

async fn get_geolocation(address: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.openrouteservice.org/geocode/search?api_key={}&text={}", api_key, address);
    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;
    Ok(resp)
}


async fn get_directions(profile: &str, start: (f64, f64), end: (f64, f64), api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.openrouteservice.org/v2/directions/{}?api_key={}&start={},{}&end={},{}", profile, api_key, start.0, start.1, end.0, end.1);
    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;
    Ok(resp)
}


fn parse_args(args: Vec<String>) -> (String, String) {
    match args.len() {
        3 => (args[1].clone(), args[2].clone()),
        _ =>  {
            println!("You need to give start and end addresses.");
            process::exit(1);
        }
    }
}


#[tokio::main]
async fn main() {
    let api_key = get_api_key();
    let args = env::args().collect();
    let (start, end) = parse_args(args);
    let start_georesult = get_geolocation(&start, &api_key).await.unwrap();
    let end_georesult = get_geolocation(&end, &api_key).await.unwrap();
    if let Some(start_point) = parse_geolocation(&start_georesult) {
        if let Some(end_point) = parse_geolocation(&end_georesult) {
            let profile = "cycling-road";
            let directions_result = get_directions(profile, start_point, end_point, &api_key).await.unwrap();
            let parsed_result = directions_result.parse::<GeoJson>().unwrap();
            println!("{:#?}", parsed_result);
        }
    }
}
