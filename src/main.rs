use std::fs;

use log::debug;

mod dom;
mod parser;
mod tools;

fn main() {
    env_logger::init();

    debug!("Initialisation du moteur François");

    debug!("Chargement du fichier de test");
    let raw_html = fs::read_to_string("./test/doctype.html").expect("Fichier non lisible");

    parser::parse(raw_html);
}
