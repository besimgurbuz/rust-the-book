mod methods;

fn main_v1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "v1: The area of the rectangle is {} square pixels",
        area_v1(width1, height1)
    );
    // The area function is supposed too calculate the area of one rectangle, but the function we wrote has two parameters,
    // and it's not clear anywhere in our program that the parameters are related. It would be more readable and more
    // manageable to group width and height together. TUPLES!
}

fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

fn main_v2_tuple() {
    let rect1 = (30, 50);

    println!(
        "v2: The area of the rectangle is {} square pixels",
        area_v2_tuple(rect1)
    );
    // Tuples don't name their elements and that makes our calculation less obvious
    // For area calculation missing the fields don't really matter but for other function it definetly matter!
    // And using right indexes for even just two fields is hard to keep in mind
}

fn area_v2_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refactoring with structs: Adding more meaning
// Let's try println! macro with our struct Rectangle,
//      println!("rect1 is {}", rect1); // ERROR: `Rectangle` doesn't implement `std::fmt::Display`
// this won't work
// The prinln! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known
// as `Display`: output intended for direct end user consumption. The primitive types we've seen so far implement `Display`
// by default, because there'se only one way you'd want to show a 1 or any other primitive type to a user. But with structs,
// the way println! should format the output is less clear because there are more display possibilities: Do you want commas
// or not? Do you want to print the curly brackets? Should all fields be shown? Due to this ambiguity, Rust doens't try to
// guess what we want
// Let's try with debug formatting
//      println!("rect1 is {:?}", rect1); // ERROR: `Rectangle` doesn't implement `std::fmt::Debug`
// Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that
// functionality available for our struct. To do that, we add the outer attribute `#[derive(Debug)]` just before the struct
// definition;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main_v3_struct() {
    let rect1 = Rectangle {
        width: 32,
        height: 50,
    };

    println!(
        "v2: The area of the rectangle is {} square pixels",
        area_v3_struct(&rect1)
    );

    println!("rect1 is {:?}", rect1); // or {:#?}
    println!("rect1 is {:#?}", rect1); // kinda formats the struct diplaying, better for large structs

    // another whay to print oout a value using the Debug format is use the `dbg!` macroo, which takes ownership of an
    // expression, prints the file and line number of where that `dbg!` macro call occurs in your code along with the resulting
    // value of that expression, and returns ownership of the value
    // NOTE: calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println! which prints
    // to the standard output console stream (stdout).

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);
}

fn area_v3_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn main() {
    main_v1();
    main_v2_tuple();
    main_v3_struct();
    methods::main_fn();
}
