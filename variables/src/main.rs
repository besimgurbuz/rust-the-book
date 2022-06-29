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
}
