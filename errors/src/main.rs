mod recoverable_errors;
mod unrecoverable_errors;

fn main() {
    //? Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such
    //? as a file not found error, we most likely just want to report the problem to the user and retry the operation.
    //? Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array,
    //? and so we want to immediately stop the program

    //? Rust has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program
    //? encounters an uncoverable error.
    unrecoverable_errors::main_fn();
    recoverable_errors::main_fn();
}
