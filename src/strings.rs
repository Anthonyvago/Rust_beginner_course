/*
Er zijn twee soorten strings;
    1) Primitive strings; de lengte hiervan is niet te bewerken.
    2) String; deze kunnen 'groeien'. Te gebruiken om te bewerken.
*/

pub fn run() {
    let /*mut*/ hello1 = "hello1"; // Primitive
    let mut hello2 = String::from("hello2 "); // Te bewerken
    
    hello2.push('W'); // Alleen voor strings.
    hello2.push_str("orld!"); // Pusht een string

    println!("len: {}", hello2.len());

    // Capacity of string in bytes.
    println!("Capacity: {}", hello2.capacity());

    // Check if string is empty:
    println!("Is empty: {}", hello2.is_empty());

    // Contains specific word or letters"
    println!("Contains 'World' {}", hello2.contains("World"));
    
    // Replace
    println!("Replace: {}", hello2.replace("World", "appelmoes"));

    // Loop through string by whitespace (spaties):
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity:
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing:
    assert_eq!(2, s.len()); // Voor testen van programma's
    assert_eq!(10, s.capacity());

    println!("{}", s)
    
    // Print both strings:
    println!("{:?}", (hello1, hello2));
}