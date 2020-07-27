use structopt::StructOpt;

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
