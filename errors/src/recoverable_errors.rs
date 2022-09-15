use core::panic;
use std::{fs::File, io::{self, ErrorKind, Read}};

pub fn main_fn() {
    // Result enum!
    let greeting_file_result = File::open("hello.txt");

    // Note that, like the Option enum, the Result enum and its variants have been brought into scope by the prelude, so
    // we don't need to specify Result:: before the Ok and Err variants in the match arms.

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Matching different errors
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // That's a lot of match expression is very useful but also much a primitive. Closures will
    // help us to clean it abit. For example above code can be written as below wit closure;
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });

    // Using match works well enough, but it can be a bit verbose and doesn't always communicate
    // intent well. The `Result<T, E>` type has many helper methods defined on it to do various,
    // more specific tasks. The `unwrap` method is a shortcut method implemented just like the 
    // match expression above. If the `Result` value is the `Ok` variant, `unwrap` will return
    // the value inside the `Ok`. If the Result is the `Err` variant, `unwrap` will call the 
    // panic! macro. Example;

    let greeting_file = File::open("hello.txt").unwrap();
    // if we run this code without a hello.txt file, a panic will be thrown
    // similarly, the `expect` method lets us also choose the panic! error message. Using expect
    // instead of unwrap and providng good error message can convey your intent and make tracking
    // down the source of a panic easier. The syntax of expect looks like this;
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    // We use expect in the same way as `unwrap`: to return the file handle or call the panic!
    // macro. the error message used by expect in tis call to panic! will be  the parameter that
    // we pass to expect, rather than the default `panic!` message that unwrap uses.
    // In production-quality code, most Rustaceans choose expect rather than `unwrap` and give
    // more context about why the operation is expected to always succeed.
    let result = propagating_errors().unwrap();
    println!("{:?}", result);
}

// When a function's implementation calls something that might fail, instead of handling the
// error within the function itself, you can return the error to the calling code so that it
// can decide what to do. This is known as propagating the error and gives more control to
// the calling code, for example code down below shows a function that reads a username from a
// file. If the file doesn't exist or can't be read, this function will return those errors to 
// the code that called the function
fn propagating_errors() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
// This pattern of propagating errors is so common in Rust that Rust provides the question mark
// operator ? to make this easier

// A Shortcut for Propagating Errors: the ? Operator

fn read_username_from_file() -> Result<String, io:Error> {
    let mut username_file = File::open("hello.txt");
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// The ? placed after a Result value is defined to work in almost the same way as the `match`
// expression we defined to handle the `Result` values in above example. If the value of the
// `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the
// program will continue. If the value is an `Err`, the `Err` will be returned from the whole
// function as if we had used the `return` keyword so the error value gets propagated to the
// calling code.
// There is a difference with ? operator: error values that have the ? operator called on them go
// through the `from` function, defined in the `From` trait in the standard library, which is used
// to convert values from one type into another. When the ? operator calls the `from` function, the
// error type received is converted into the error type defined in the return type of the current
// function.
// For example, we could change the read_username_from_file function above to return a 
// custom error type named OurError that we define. If we also define impl From<io::Error> for 
// OurError to construct an instance of OurError from an io::Error, then the ? operator calls in 
// the body of read_username_from_file will call from and convert the error types without needing 
// to add any more code to the function.

// The ? operator eliminates a lot of boilerplate and makes this function's implementation simpler.
// We could even shorten this code further by chaning method calls immediately after the ?, as
// shown;
fn read_username_from_file_shortened() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username);

    Ok(username)
}
