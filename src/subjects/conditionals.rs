/*
Conditionals worden gebruikt om de conditie te controleren en aan de hand van het resultaat iets te doen.
*/

pub fn run() {
    println!();
    println!("*********************");
    println!("***** conditionals.rs *****");
    println!("*********************");

    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand if:
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
