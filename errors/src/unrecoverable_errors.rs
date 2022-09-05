use core::panic;

pub fn main_fn() {
    // Sometimes, bad things happen in your code, and there's nothing you can do about it. In these cases, Rust has the
    // panic! macro. By default, these panics will point a failure message, unwind, clean up the stack, and quit. Via an
    // environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track
    // down the source of the panic.

    // ? Unwinding the Stack or Aborting in Response to a Panic
    // ? By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleanup
    // ? is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the
    // ? program without cleaning up. OS should clean the used memory by program. To enable aborting;
    // in Cargo.toml
    // [profile.release]
    // panic = 'abort'

    // calling panic
    // panic!("crash and burn");
}
