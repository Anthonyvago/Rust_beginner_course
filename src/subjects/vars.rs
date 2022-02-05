// Standaard is een variabele niet te veranderen.

pub fn run() {
    println!();
    println!("*********************");
    println!("***** vars.rs *****");
    println!("*********************");

    let name = "Brad"; // Niet te wijzigen

    let mut age = 37; // Wel te wijzigen
    age += 1;

    println!("My name is {} and I am {}", name, age);

    const ID: i32 = 001; // Const variabelen
    println!("ID: {}", ID);

    // Meerdere variabelen tegelijk:
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}