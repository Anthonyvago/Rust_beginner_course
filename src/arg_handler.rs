#[path = "subjects/arrays.rs"] mod arrays;
#[path = "subjects/cli.rs"] mod cli;
#[path = "subjects/conditionals.rs"] mod conditionals;
#[path = "subjects/enums.rs"] mod enums;
#[path = "subjects/functions.rs"] mod functions;
#[path = "subjects/loops.rs"] mod loops;
#[path = "subjects/pointer_ref.rs"] mod pointer_ref;
#[path = "subjects/strings.rs"] mod strings;
#[path = "subjects/structs.rs"] mod structs;
#[path = "subjects/tuples.rs"] mod tuples;
#[path = "subjects/types.rs"] mod types;
#[path = "subjects/vars.rs"] mod vars;
#[path = "subjects/vectors.rs"] mod vectors;

use std::env;

/**
 * Deze methode kijkt of er maar één argument meegegeven is.
 * Als er geen argument is meegegeven is de lengte 1 (pad van .exe).
 */
fn check_one_arg_given() -> bool {
    env::args().len() == 1
}

/**
 * Deze methode voert de run()-functie uit (elk .rs-bestand bevat een
 * run()-functie) per meegegeven bestand.
 */
pub fn arg_handler() {
    if check_one_arg_given() {
        println!("Geef één of meer .rs-bestanden mee als argument.");
        println!("Deze bestanden zijn te vinden in de 'subjects'-map.");
    }

    for arg in env::args() {
        if cli::valid_arg(arg.clone()) {
            match arg.as_str() {
                "arrays.rs" => arrays::run(),
                "cli.rs" => cli::run(),
                "conditionals.rs" => conditionals::run(),
                "enums.rs" => enums::run(),
                "functions.rs" => functions::run(),
                "loops.rs" => loops::run(),
                "pointer_refs.rs" => pointer_ref::run(),
                "strings.rs" => strings::run(),
                "structs.rs" => structs::run(),
                "tuples.rs" => tuples::run(),
                "types.rs" => types::run(),
                "vars.rs" => vars::run(),
                "vectors.rs" => vectors::run(),
                _ => println!("Argument '{}' is ongeldig!", arg),
            }
        }
    }
}