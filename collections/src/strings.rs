pub fn strings_main() {
    // what is a String?
    // Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed
    // form `&str`.

    // The `String` type, which is provided by Rust's standard library rather than coded into the core langeugae, is a growable,
    // mutable, owned UTF-8 encoded string type. Both `String` and string slices are UTF-8 encoded.

    // Creating new String
    // Many of the same operation available with Vec<T> are available with `String` as well, because `String` is actually
    // implemented as a wrapper around a vector of bytes with some extra guarantees restrictions, and capabilities. An example
    // of a function that works the same way with Vec<T> and String is the new function to create an instance;

    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // String::from
    let s = String::from("initial contents");

    // Updating a String
    // A `String` can grow in size and its contents can cahgne, just like the contents of a Vec<T>, if you push more data into it
    // In addition, you can conveniently use the `+` operator or the `format!` macro to concatenate `String` values.

    let mut s = String::from("foo");
    s.push_str("bar");
    // after `push_str` s will contain `foobar`. The push_str method takes a string slice because we don't necessarily want to
    // take ownership of the parameter. for example, in the code below we want to be able to use `s2` after appending its
    // contents to `s1`
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // The `push` method takes a single character as a parameter and adds it to the String.
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    // Concatenation with the `+` operator or the `format!` macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // The reason `s1` is no longer valid after the addition, and the reason we used a reference to `s2`, has to do with the
    // signature of the method that's called when we use the `+` operator. The `+` operator uses the `add` method, whose
    // signature looks something like this:
    // fn add(self, s: &str) -> String {}
    // In the standard library, you'll see `add` defined using generics and associated types. Here, we've substituted in
    // concrete types, which is what happens when we call this method with `String`

    // First, s2 has an `&`, meaning that we're adding a reference of the second string to the first string. This is because
    // of the `s` parameter in the `add` function: we can only add a `&str` to a String; we can't add two `String` values
    // together.
    // Compiler coercing the &String argument into a &str (turs &s2 to &s2[..])

    // Second, we can see in the signature that `add` takes ownership of `self`, because `self` does not have an `&`. This
    // means `s1` above will be moved into the `add` call and will no longer be valid after that. So although let s3 =s1 + &s2
    // looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a
    // copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it's making a lot
    // of copies but isn't; the implementation is more efficient than copiying.

    // if we need concatenate multipe strings, the behavior of the `+` operator gets unwieldy:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("triple concatination with +: {s}");

    // much more readable
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("triple concatination with format!: {s}");

    // indexing into Strings
    let s1 = String::from("hello");
    // let h = s1[0]; // ERROR: the type `String` cannot be indexed by `{integer}`

    // solution: Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {}", s);
    // Here, s will be a &str that contains the first 4 bytes of the string. But each of these characters was 2 bytes, which
    // means `s` will be Зд`
    // if we were to try to slice only part of a character's bytes with something like &hello[0..1], Rust would panic at
    // runtime in the same way as if an invalid index were accessed in a vector:
    // &hello[0..1]; // ERROR

    // Methods for iterator over Strings

    println!("____CHARS____");
    for c in "Зд".chars() {
        println!("{}", c);
    }
    println!("____BYTES____");
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
