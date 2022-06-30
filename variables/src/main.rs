use std::io;

// Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
const THREE_HOOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("Three hours is equals {} seconds.", THREE_HOOURS_IN_SECONDS);

    // Shadowing
    let a = 5;
    let a = a + 3;

    {
        let a = a * 2;
        println!("The value of a is in the inner scope is: {}", a);
    }
    println!("The value of a is: {}", a);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x: {} y: {}", x, y);

    // divition
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // floored becaused the default type is i32
    println!("quotient: {} floored: {}", quotient, floored);

    // chart
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c is a chart which holds: {}", c);
    println!("z: {}", z);
    println!("chart's can also holds some emojis! {}", heart_eyed_cat);

    /*
        Compound Types
        Can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

        The tuple type
        A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
        Tuples have a fixed length: once declared, they cannot grow or shrink in size.

        We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type,
        and the types of the different values in the tuple don't have to be the same.
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple holds (x, y, z) => {:?}", tup);
    // destruct like javascript
    let (x, y, z) = tup;
    println!("x: {} y: {} z: {}", x, y, z);

    // tuple variables are also accessiable via period
    let five_hundred = tup.0;
    println!("five hundred: {five_hundred}");
    /*
        # The tuple without any values has a special name, unit. This value and its corresponding type are both written () and
        represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.
    */

    /*
        The array type
        Another way to have a collection of multiple value is with an array. Unlike a tuple, every element of an array must have the *same type*.
        Unlike arrays in some other languages, arrays in Rust have a fixed length.
        We write the values in an array as a comma-separated list inside square brackets:
    */
    let a = [1, 2, 3, 4, 5];
    println!("array a holds: {:?}", a);

    /*
        Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed
        number of elements. An array isn't as flexiable as the vector type, though. A vector is a similar collection type provided by the standard library
        that is allowed to grow or shrink in size. If you're unsure whether to use an array or a vector, chances are you should use a vector.

        However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month
        in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:
    */
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The months in a year are: {:?}", months);

    // array first type then length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("the array a holds: {:?}", a);

    /*
        You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon,
        and then the length of the array in square brackets, as shown here:
    */
    let five_threes = [3; 5];
    println!("five three array would be like this => {:?}", five_threes);
    guess_the_index();
}

// invalid array element access
fn guess_the_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
