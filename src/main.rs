/**
 * Dit project is gemaakt met behulp van de volgende video: https://www.youtube.com/watch?v=zF34dRivLOw.
 * Ik (zie auteur) heb deze video in zijn volledigheid gevolgd en kan hierdoor aantonen dat ik de basis
 * van rust begrijp en zelf kan implementeren. Dit is te zien door het hieronderstaande uit te voeren of
 * door te kijken naar "arg_handler.rs". 
 *
 * Hoe te gebruiken?
 * In de 'subjects'-map (te vinden in de src) zijn er 13 rust-bestanden geschreven aan de hand van bovenstaande video.
 * Voer het volgende uit in de root van dit project: cargo run <.rs-bestand>.
 * Er kunnen ook meerdere .rs-bestanden worden meegegeven. Voorbeeld: 'cargo run loops.rs arrays.rs'.
 */
mod arg_handler;

fn main() {
    arg_handler::arg_handler();
}
