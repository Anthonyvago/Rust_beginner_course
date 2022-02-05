/**
 * Reference pointers zijn adressen in de memory van waar een variabele is opgeslagen.
 */

pub fn run() {
    println!();
    println!("*********************");
    println!("***** pointer_refs.rs *****");
    println!("*********************");

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;   // Met het kopiëren van een vector, moet je een reference gebruiken
                        // aangezien een vector een non-primitive datatype is.

    println!("Values:");
    println!("{:?}", vec1);
    println!("{:?}", vec2);
}
