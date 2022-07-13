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

    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hodl the contents. This means:
    // - The memory must be requested from the memory allocator at runtime.
    // - We need a way of returning this memory to the allocator when we're done with our String.

    // The first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

    // However, the second part is different. In languages with `garbage collector (GC)`, the GC keeps track of and cleans up memory that isn't being used anymore, and we don't need to think about it. In most languages
    // without a GC, it's our responsibility to identify when memory is no longer being used and call code to explicity free it, just as we did to request it. Doing this correctly has historically been a difficult
    // programming problem. If we forget, we'll waste memory. If we don it early, we'll have an invalid variable. if we do it twice, that'a a bug too. We need to pair exactly one allocate with exactly one free.

    // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
}
