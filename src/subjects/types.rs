/* 
    Integers: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128.
    Floats: f32, f64
    Boolean: (bool)
    Characters: (char)
    Tuples zijn eigenlijk lists.
    Arrays zijn vaste lengtes.
*/


/* 
Rust is een statische programmeertaal. Dit betekent dat het tijdens 
compile-time de datatypes moet weten van elke variabele. Het is niet
verplicht om bij een variabele een datatype aan te geven aangezien
de compiler kan inschatten welke dit is.
*/

pub fn run() {
    println!();
    println!("*********************");
    println!("***** types.rs *****");
    println!("*********************");

    let x = 1; // Standaard is dit een i32.

    let y = 2.5; // Standaard is dit een f64.

    let z: i64 = 4545454545; // Dit is nu een i64.

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    let is_active: bool = true;

    let is_greeter: bool = 10 < 5;

    let a1 = 'a'; // Char is met single quotes.
    let face = '\u{1F970}'; // emoji zijn ook unicodes. Google naar emoji unicodes.

    println!("{:?}", (x, y, z, is_active, is_greeter, a1, face))
}