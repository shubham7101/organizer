use std::process;
use organizer::{ run, config };

fn main() {
    let config = match config::Config::load("config.yaml") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        },
    };

    println!("{config:#?}");

    run(&config);
}
