pub fn main_fn() {
  println!("\nREFERENCES AND BORROWING");
  // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
  let s1 = String::from("hello");
  let len = calculate_length(&s1);

  println!("The length of '{s1}' is {len}");
  // referencing operator (&)
  // dereferencing operator (*)

  // We call the action of creating a reference `borroowing`. As in real life, if a person owns something, you can borrow it from them. When you're done, you have to give it back. You don't own it.
  // So you cannot mutate the reference unless it's a `mut reference`

  // Mutable References
  let mut s = String::from("hello");
  change(&mut s);
  println!("{s}");

  // mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
  // let r1 = &mut s;
  // let r2 = &mut s; // ERROR

  // The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these behaviors occur:
  // - Two or more pointers access the same data at the same time
  // - At least one of the pointers is being used to write to the data
  // - There's no mechanism being used to synchronize access to the data
  // We can use curly brackets to create a new scope, alloowing for multiple mutable references, just not simultaneous ones:
  // {
  //   let r1 = &mut s;
  // }
  // let r2 = &mut s;

  // Rust enforces a similar rule for combining mutable and immutable references;
  // let r1 = &s; // no problem
  // let r2 = &s; // no problem
  // let r3 = &mut s; // BIG PROBLEM
  // println!("{}, {}, and {}", r1, r2, r2);
  // Whew! we also cannot have a mutable reference while we have an immutable one to the same value.
}

fn calculate_length(s: &String) -> usize {
  // s is a reference to a String
  s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped

fn change(some_string: &mut String) {
  some_string.push_str(", world!");
}
