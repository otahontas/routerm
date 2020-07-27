use std::fmt;

use crate::step::Step;

pub struct Route {
    distance: f64,
    duration: f64,
    steps: Vec<Step>
}

impl Route {
    pub fn new(distance: f64, duration: f64, steps: Vec<Step>) -> Route {
		Route { distance, duration, steps }
    }
}

impl fmt::Display for Route{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut write = String::from("");
        write.push_str(&format!("Distance: {} km\n", self.distance / 1000.0));
        write.push_str(&format!("Duration: {} min\n\n", (self.duration / 60.0).round()));
        self.steps.iter()
			.for_each(|s| write.push_str(&format!("{}\n", s)));
       
		write!(f, "{}", write)
    }
}

