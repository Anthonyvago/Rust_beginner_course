/*
Tuples groeperen waarden van verschillende datatypes met een max aantal van 12.
*/

pub fn run() {
    println!();
    println!("*********************");
    println!("***** tuples.rs *****");
    println!("*********************");

    let person: (&str, &str, u8) = ("Brad", "Mass", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}