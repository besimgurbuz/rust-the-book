fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    scope_block_expression();

    let x = five();

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*
    Statements and Expressions

    Statements are instructions that perform soome action and do not return a value. Expressions evaluate to a resulting value.

    *Creating a variable and assigning a value to it with the `let` keyword is a statement.
        let y = 6;

    *Function definitions are also statements; the entire preciding example is statement in itself.
    Statements do not return values. Therefore, you can't assign a let statement to another variable, as the following code tries to do; you'll get an error:
        let x = (let y = 6);
    The let y = 6; statement does not return a value, so there isn't anything foor x too bind to. This is different from other languages,
    such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write x = y = 6 and have both x and y
    have the value 6; that is not the case in Rust.

    Expressions evaluate to a value and make up most of the code that you'll write in Rust. Consider a math operation, such as 5 + 6,
    which is an expression that evaluates to value 11. Expressions can be part of statements; for example let y = 6; `6` is an expression that evaluates to value 6.
    Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example;
*/
fn scope_block_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
/*
    This expression:
    {
        let x = 3;
        x + 1;
    }
    is a block that, in this case evaluates to 4. That value gets bound to y as part oof the `let` statement. Note that the `x + 1` line doesn't have a semicolon at the end,
    unlike most of the lines you've seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of expression, you turn it into a statement,
    and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

    Functions with return values
    Functions can return values to the code that calls them. We don't name return values, but we must declare their type after an arrow (->). In Rust, the return value of the
    function is synonymous with the value of the final expression in the block of a function. You can return early from a function by using the return keyword and specifying a value,
    but most functions return the last expression implicitly. Here's an example of a function that returns a value:
*/
fn five() -> i32 {
    5
}
/*
    There are no function calls, macros or even let statements in the five function- just the number 5 by itself. That's a perfectly valid function in Rust. Note that the function's return type
    is specified too, as -> i32.
*/
