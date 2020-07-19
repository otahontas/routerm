use std::fmt;

pub struct Step {
    distance: f64,
    instruction: String,
}

impl Step {
    pub fn new(distance: f64, instruction: String) -> Step {
        Step { distance, instruction}
    }
}


impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.instruction.starts_with("Arrive") {
            write!(f, "{}", self.instruction)
        } else {
            write!(
                f,
                "{}, after {} m",
                self.instruction,
        		self.distance.round()
    		)
        }
	}
}
