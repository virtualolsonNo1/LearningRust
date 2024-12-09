use rand::Rng;
enum DiceRoll {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl TryFrom<i32> for DiceRoll {
    // We need to specify what type of error we'll return if the conversion fails
    type Error = String;

    // This function defines how to convert from i32 to DiceRoll
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DiceRoll::One),
            2 => Ok(DiceRoll::Two),
            3 => Ok(DiceRoll::Three),
            4 => Ok(DiceRoll::Four),
            5 => Ok(DiceRoll::Five),
            6 => Ok(DiceRoll::Six),
            _ => Err(format!("Invalid dice value: {}", value))
        }
    }
}

fn main() {
    let mut rng: DiceRoll = DiceRoll::try_from(rand::thread_rng().gen_range(1..7)).expect("This should never fail");
    match rng {
        DiceRoll::Three => println!("Three, so get fancy hat!!!!!!"),
        DiceRoll::Six => println!("Six, so lose fancy hat!!!!!!"),
        _ => println!("Reroll!!!!!"), 
    }
}


