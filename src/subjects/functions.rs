/**
 * Functions are used to store blocks of code for re-use.
 */

pub fn run() {
    println!();
    println!("*********************");
    println!("***** functions.rs *****");
    println!("*********************");

    greeting("Hello", "Jane");

    // Bind function values to variables:
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure:
    let nr3: i32 = 10; // Onderstaande closure functie kan ook nr3 meenemen in de functie-berekening.
    let add_nums = |nr1: i32, nr2: i32| nr1 + nr2 + nr3;
    println!("Closure sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(nr1: i32, nr2: i32) -> i32 {
    nr1 + nr2
}
