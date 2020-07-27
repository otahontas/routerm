use std::env;
use std::process;
use geojson::GeoJson;
use geojson::Value::Point;
use structopt::StructOpt;

use crate::route::Route;
use crate::step::Step;
mod route;
mod step;

#[derive(StructOpt)]
#[structopt(name = "Arguments", about = "Arguments parsed from command line.")]
struct Args {
    #[structopt(short, long)]
    start: String,

    #[structopt(short, long)]
    end: String,

    #[structopt(short, long, possible_values = &["driving-car", "driving-hgv", "cycling-regular", "cycling-road", "cycling-mountain", "cycling-electric", "foot-walking", "foot-hiking", "wheelchair"],
    case_insensitive = true)]
    profile: String,
}

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

#[tokio::main]
async fn main() {
    let args = Args::from_args();
    let api_key = get_api_key();
    let start_georesult = get_geolocation(&args.start, &api_key).await.unwrap();
    let end_georesult = get_geolocation(&args.end, &api_key).await.unwrap();

    if let Some(start_point) = parse_geolocation(&start_georesult) {
        if let Some(end_point) = parse_geolocation(&end_georesult) {
            let directions_result = get_directions(&args.profile, start_point, end_point, &api_key).await.unwrap();
            let parsed_result = directions_result.parse::<GeoJson>().unwrap();

            println!("Start:\t  {}\nEnd:\t  {}", args.start, args.end);
            match parse_geojson(parsed_result) {
                Some(x) => println!("{}", x),
                None    => println!("Couldn't parse geojson")
            
        }
    }
    
    }
    
}

fn parse_geojson(geojson: GeoJson) -> Option<Route> {
    match geojson {
        GeoJson::FeatureCollection(fc) => {
            let props= fc.features[0]
            			  .properties.clone()?;
            let segs = &props.get("segments")?[0];
            let steps: Vec<Step> = segs.get("steps")?
                .as_array()?.iter()
                .map(|s| Step::new( // can't use ? inside closure, so unwrap is used
                    		s.get("distance").unwrap().as_f64().unwrap(),
                    		String::from(s.get("instruction").unwrap().as_str().unwrap())))
                .collect();

            let r = Route::new(
                segs.get("distance")?.as_f64()?,
                segs.get("duration")?.as_f64()?,
                steps);

			Some(r)},
        _ => None
    }
}
