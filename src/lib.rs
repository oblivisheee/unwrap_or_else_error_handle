/// Function to handle errors in a way that prints a message and exits the program.
/// result.unwrap_or_else(handle_error("An error occurred"))
/// Prints "An error occurred: <error message>" and exits on error.
pub fn handle_error<T, E: std::fmt::Display>(msg: &'static str) -> impl FnOnce(E) -> T {
    move |e| {
        eprintln!("{}: {}", msg, e);
        std::process::exit(1);
    }
}
