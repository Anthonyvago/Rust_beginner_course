// CLI = Command Line Interface

use std::env;

pub fn run() {
    println!();
    println!("*********************");
    println!("***** cli.rs *****");
    println!("*********************");

    // Eerste element in de onderstaande array is altijd het pad naar de executable.
    let args: Vec<String> = env::args().collect();
    let command = args[2].clone();
    let name = "Brad";
    let status = "100%";

    // println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command!");
    }
}

/**
 * Controleert of het argument "target" bevat
 * aangezien het eerste argument altijd het pad
 * is naar de executable. Dit pad bevat altijd het
 * woordje "target". Hierdoor kunnen wij filteren
 * op alleen onze meegegeven argumenten.
 */
pub fn valid_arg(arg: String) -> bool {
    !arg.contains("target")
}