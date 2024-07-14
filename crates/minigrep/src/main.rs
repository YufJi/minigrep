use std::env;
use std::process;

use minigrep;

fn main() {
    let start_time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

    let elapsed = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}
