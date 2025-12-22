const PERSONAL_TRAINER: &str = "Carl Cardio";

pub fn ask_about_trainer() {
    println!("The cardio trainer is {PERSONAL_TRAINER}");
}

#[derive(Debug)]
pub enum CardioTool {
    TreadMill,
    Bike,
}

#[derive(Debug)]
pub struct Exercise {
    day: String,
    tool: CardioTool,
    minutes: u32,
}

impl Exercise {
    pub fn new(day: String, tool: CardioTool, minutes: u32) -> Exercise {
        Exercise {
            day, tool, minutes
        }
    }
}