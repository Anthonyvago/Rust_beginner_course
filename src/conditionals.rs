/*
Conditionals worden gebruikt om de conditie te controleren en aan de hand van het resultaat iets te doen.
*/

pub fn run() {
    let age: u8 = 30;
    let check_id:bool = true;
    let knows_person_of_age = true;

    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }
}