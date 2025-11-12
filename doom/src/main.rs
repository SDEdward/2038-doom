use std::env;

/// Entry point of the application.
///
/// The application expects a command-line argument to activate a countdown feature.
///
/// # Arguments
///
/// * `c`, `count`, or `countdown`: Activates the countdown feature.
///
/// # Usage
///
/// To compile the application:
/// ```bash
/// cargo build
/// ```
///
/// To run the application with countdown:
/// ```bash
/// cargo run -- count
/// ```
///
/// If an invalid argument is provided, an error message will be printed to the standard error output.
pub fn main() {
    let args: Vec<String> = env::args().collect();

    let _countdown = match args.get(1).map(String::as_str) {
        None => false,
        Some("c") | Some("count") | Some("countdown") => true,
        Some(arg) => {
            eprintln!("Incorrect argument: {}", arg);
            eprintln!("Usage: cargo run -- [c|count|countdown]");
            return;
        }
    };

    doom_2038::doom(false);
}
