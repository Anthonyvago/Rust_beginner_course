/*
* Enums zijn types die vooraf een aantal waardes kunnen hebben.
*/

 /**
  * We simuleren de mogelijke bewegingen van een avatar in een spel.
  */
 enum Movement {
    Up,
    Down,
    Left,
    Right
 }

 fn move_avatar(m: Movement) {
    // Perform action depending on info.
    match m {
        Movement::Up => println!("Avatar moving up."),
        Movement::Down => println!("Avatar moving down."),
        Movement::Left => println!("Avatar moving left."),
        Movement::Right => println!("Avatar moving right.")
    }
 }

pub fn run() {
    println!();
    println!("*********************");
    println!("***** enums.rs *****");
    println!("*********************");

    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}