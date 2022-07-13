mod references_and_borrowing;

fn main() {
  // Variable Scope
  // A scope is the range within a program for which an item is valid. Take the following variable:
  {
    // s is not valid here, it’s not yet declared
    let _s = "hello"; // s is valid from this point forward

    // do stuff with s
  } // this scope is now over, and s is no longer valid
    // The variable s refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it's declared until the end of the current scope.

  // The String Type
  // The all types that known size can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make new, independent instance if another part of code needs to use
  // the samce value in a different scope. But we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the String type is great example.

  // String literals are convenient, but they aren't suitable for every situation in which we may want too use text. One reason is that they're immutable. Another is that not every string value can be known when we write
  // our code: for example, what if we want to take user input and store it? For these situations, Rust has a second string type, `String`. This type manages data allocated on the heap and as such is able to store an amount
  // of text that is unknown to us at compile time. You can create a String from a string literal using the `from` function, like so:
  let mut s = String::from("hello");
  // This  kind of string can be mutated:
  s.push_str(", world!");
  println!("{}", s);

  // Memory and Allocation
  // In the case of a string literal, ("text") we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only
  // come from the string literal's immutability. Unfortunately, we can't put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program

  // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
  // - The memory must be requested from the memory allocator at runtime.
  // - We need a way of returning this memory to the allocator when we're done with our String.

  // The first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

  // However, the second part is different. In languages with `garbage collector (GC)`, the GC keeps track of and cleans up memory that isn't being used anymore, and we don't need to think about it. In most languages
  // without a GC, it's our responsibility to identify when memory is no longer being used and call code to explicity free it, just as we did to request it. Doing this correctly has historically been a difficult
  // programming problem. If we forget, we'll waste memory. If we don it early, we'll have an invalid variable. if we do it twice, that'a a bug too. We need to pair exactly one allocate with exactly one free.

  // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
  {
    let _s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
  } // this scope is now over, and s is no longer valid
    // There is a natural point at which we can return the memory oour String needs to the allocator: when _s goes out of scope. When a variable goes out of scooope, Rust calls a special function for us. This function is called
    // `drop`, and it's where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

  // This pattern has a profound impact oon the way Rsut code is written. It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the
  // data we've alloocated on the heap. Let's explore some of those situations now.

  // Ways Variables and Data Interact: Move
  // Multiple variables can interact with the same data in different ways in Rust. Let's look at an example using an integer;
  let x = 5;
  let y = x;
  // We can probably guess that what this is doing: "bind the value 5 to x; then make a copy of the value in x and bind it to y." We now have two variables, x and y, and both equal 5. This is indeed what is happening, because
  // integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack. Let's look at the String version;
  let s1 = String::from("hello");
  let s2 = s1;
  // This looks very similar, so we might assume that the way it works would be the same: that is, the second line would make a copyof the value in s1 and bind it to s2, but this isn't quite what happens.
  // Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop functioon and cleans up the heap memory for that variable. This is a problem when s2 and s1 go out of scope, they will both try
  // to free the same memory. This is known aas a double free error and is one of the memory safety bugs. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
  // To ensure memory safety, after the line `let s2 = s1`, Rust considers s1 as no longer valid. Therefore, Rust doesn't need to free anything when s1 goues out of scope. Using s1 won't work;
  // s1.capacity(); // ERROR

  // In addition, there's a design choice that's implied by this: Rust will never automatically create "deep" copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance

  // Ways Variables and Data Interact: Clone
  // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called `clone`. Here's an example;
  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("s1 = {s1} s2 = {s2}"); // This works just fine.
                                   // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It's a visual indicator that something different is going on.

  // Stack-Only Data: Copy
  // There’s another wrinkle we haven’t talked about yet. This code using integers
  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x, y);
  // But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.
  // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent
  // x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying and we
  // can leave it out.

  // Rust has a special annotation called the `Copy` trait that we can place on types that re stored on the stack, as integers are. If a type implements the `Copy` trait, variables that use it do not move, but rather are
  // trivially copied, making them still valid after assignment to another variable.

  // Rust won't let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait. If the type needs something special to happen when the value goes out of scope and we add the `Copy`
  // annotation to that type, we'll get a compile-time error.
  // So what types implement the `Copy` trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement `Copy`, and nothing that requires allocation
  // or is some form of resource can implement `Copy`. Here are some of the types that implement Copy:
  // - All the integer types, such as `u32`
  // - The boolean type, `bool`, with values `true` and `false`
  // - All the floating point types, such as `f64`
  // - The character type, `char`
  // - Tuples, if they only contain types that also implement `Copy`. For example `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

  // Ownership and functions
  // The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.
  {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function
                        // ... and so is no longer valid here
                        // s.push_str(", world"); // ERROR

    let x = 5; // x coomes into scope
    makes_copy(x) // x would move into the function,
                  // but i32 is Copy, so it's okay to still
                  // use x afterward
  } // Here, x goes out of scope then s. But because s's value was moved, nothing special happens.

  // Return values and scope
  // Returning values can also transfer ownership;
  {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 moved into takes_and_gives_back, which also moves its return value into s3
  } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens, s1 goes out of scope and is dropped.

  // While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It's quite annoying that anything we pass in also
  // needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

  // Rust does let us return multiple values using a tuple;
  let s1 = String::from("hello");
  let (s2, len) = calculate_length(s1);
  println!("The length of '{s2}' is {len}");

  // But this is too much ceremony and lot of work for a concept that sould be common. Luckily for us, Rust has a feature for using a value withou transferring ownership, called references.

  references_and_borrowing::main_fn();
}

fn takes_ownership(some_string: String) {
  // some_string comes into scope
  println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
  // some_integer comes into scope
  println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
  // gives_ownership whill move its return value into the function that calls it
  let some_string = String::from("yours"); // some_string comes into scope
  some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
  // a_string comes into scope
  a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}
