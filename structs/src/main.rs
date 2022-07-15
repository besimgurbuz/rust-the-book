use std::fmt;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(is_active: {}, username: {}, email: {}, sign_in_count: {})",
            self.active, self.username, self.email, self.sign_in_count
        )
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    // Note that the entire instace must be mutable; Rust doesn't allow us to mark only certain fields as mutable. As with any
    // expression.

    // Creating instances from other instances with struct update syntax
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // using struct update syntax, we can achive the same effect with less code;
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // ownership of user1's email stay still but username field is moved to user2
    // it means we cannot use `user1.username anymore`
    println!("user2: {} user1.email: {}", user2, user1.email);

    // Using tuple structs without named fields to create different types
    // Rust also suppoorts structs that look similar to tuples, called tuple structs. Tupe structs have the added meaning
    // the struct name provides but don't have names associated with their fields; rahter they just have the types of the
    // fields; rather they just have the types of the fields. Tuple structs are useful when you want too give the whole tuple
    // a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be
    // verbose or redundant
    // Defining tuple struct;
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    // _black and _origin values are different types, because theyre instances of different tuple structs. NO DUCK TYPING!

    // Unit-Like Structs without any fields
    // You can also define structs that don't have any fields! These are called unit-like structs because they behave
    // similarly too `()` (unit type). Unit-like structs can be useful when you need to implement a trait on some type but
    // don't have any data that you want to store in the type itself. Example;
    struct AlwaysEqual;
    // no need for curly brackets or parantheses!
    let subject = AlwaysEqual;

    // Ownership of Struct Data
    // In the User struct definition, we used the owned String type rather than the &str string slice type. This is a
    // deliberate choice because we want each instace of this struct to own all of its data and for that data to be valid
    // for as long as the entire struct is valid

    // It's also possible for structs to store references to data owned by something else, but to do so requires the use
    // of `litetime`. Lifetimes ensure that the data referenced by struct is valid for as long as the struct is. Let's say
    // you try to store a reference in a struct without specifiying lifetimes, like the following
    // WONT WORK! WITHOUT LIFETIME
    // struct User {
    //     active: bool,
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    // }

    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "someusername",
    //     active: true,
    //     sign_in_count: 1
    // };
}

// we can construct new instance of the struct as the last expression in the function body to implicity return that new
// instance
fn build_user(email: String, username: String) -> User {
    // Using the Field Init Shorthand
    // when the parameter names and the struct field names are exactly the same, we can use the field init shorthand
    // instead of `email: email` just `email`
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
