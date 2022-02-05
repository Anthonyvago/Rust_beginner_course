/**
 * Structs worden gebruikt om custom datatypes te maken .
 */

// Traditionele struct:
struct ColorTraditional {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct:
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct the person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name:
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name:
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple:
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    println!();
    println!("*********************");
    println!("***** structs.rs *****");
    println!("*********************");

    // Traditional struct:
    let mut traditional = ColorTraditional {
        red: 255,
        green: 0,
        blue: 0,
    };

    traditional.red = 200;
    println!("Color {} {} {}", traditional.red, traditional.green, traditional.blue);

    // Tuple struct:
    let mut tuple = ColorTuple(255, 0, 0);
    tuple.0 = 200;
    println!("Color {} {} {}", tuple.0, tuple.1, tuple.2);

    // Functions in a struct
    let mut p = Person::new("John", "Doe");

    println!("Person: {}", p.full_name());
    
    p.set_last_name("Williams");
    
    println!("Person: {}", p.full_name());
    println!("Person Tuple: {:?}", p.to_tuple());
}
