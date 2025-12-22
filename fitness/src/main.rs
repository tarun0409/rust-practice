mod cardio;
mod weightlifting;

use cardio::CardioTool;
use cardio::Exercise as CardioExercise;
use weightlifting::Exercise as WeightliftingExercise;
mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {NUTRITIONIST}");
    }
}
#[derive(Debug)]
struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    fn new() -> GymWorkout {
        diet::ask_about_program();
        cardio::ask_about_trainer();
        weightlifting::ask_about_program();
        GymWorkout { cardio: CardioExercise::new(String::from("Running"), CardioTool::TreadMill, 30), weightlifting: WeightliftingExercise::new(String::from("Dumbell curl"), 20) }
    }
}

fn main() {
    println!("{:?}", GymWorkout::new())
}
