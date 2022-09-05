use core::panic;
use std::{fs::File, io::ErrorKind};

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
}
