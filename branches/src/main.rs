fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match expressions.
    // Condition of if (or else if) block must be a bool.

    // Using if in a let statement
    // Because if is an expression, we can use it on the right side of a let statement to assign the outcoome to a variable;
    let condition = true;
    // *Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // If the types are mismatched, as in the following example, we'll get an error:
    // let number = if condition { 5 } else { "six" };  // ERROR
}
