fn main() {
    // Rust has three kind of loops: loop, while and for. Lets try each one.

    // loop
    // The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
    // loop {
    //     println!("again!");
    // }

    // Returning values from loops
    // One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the
    // loop to the rest of your code. To do this, you can add the value you want to returned after the break expression you use to stop the loop; that value will be returned out of the loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels to disambiguate between multiple loops
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count = {count}");

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // *We'we now increased the safety of the code and eliminated the cance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.

    // *The safetly and conciseness of for loops make them the most coommonly used loop construct in Rust. Even in situations in which you want to run some code a certain number of times, most Rustaceans would use
    // *a for loop. The way to do that would be to use a Range, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.
    // Here's what the countdown would look like using a for loop;
    for number in 1..4 {
        println!("{number}!");
    }
}
