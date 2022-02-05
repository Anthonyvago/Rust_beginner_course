mod subjects;

use subjects::arrays;
use subjects::cli;
use subjects::conditionals;
use subjects::enums;
use subjects::functions;
use subjects::loops;
use subjects::pointer_ref;
use subjects::strings;
use subjects::structs;
use subjects::tuples;
use subjects::types;
use subjects::vars;
use subjects::vectors;

use std::env;

fn argHandler() {
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

fn main() {
    argHandler(); 
}
