use structopt::StructOpt;
use std::env;
use std::process;

#[derive(StructOpt)]
#[structopt(name = "Arguments", about = "Arguments parsed from command line.")]
pub struct Args {
    #[structopt(short, long)]
    start: String,

    #[structopt(short, long)]
    end: String,

    #[structopt(short, long, possible_values = &["driving-car", "driving-hgv", "cycling-regular", "cycling-road", "cycling-mountain", "cycling-electric", "foot-walking", "foot-hiking", "wheelchair"],
    case_insensitive = true)]
    profile: String,
}

impl Args {
    pub fn get_args() -> (String, String, String) {
        let args = Args::from_args();
        (args.start, args.end, args.profile)
    }
}

pub fn get_api_key() -> String {
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