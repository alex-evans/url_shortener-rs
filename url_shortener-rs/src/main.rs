
use std::process;

use todo_rs::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = url_shortener_rs::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}