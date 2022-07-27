pub fn vectors_main() {
  // Vector
  let v: Vec<i32> = Vec::new();
  println!("{:?}", v);
  // with default values
  let mut v = vec![1, 2, 3];
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  println!("{:?}", v);

  {
    let v = vec![1, 2, 3, 4];

    // do stuff with v
  } // <- v goes out of scope and is freed here
    // when the vector gets dropped, all of its contents are also dropped

  let v = vec![1, 2, 3, 4, 5];
  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  match v.get(20) {
    Some(el) => println!("the 20th el is {}", el),
    None => println!("there is no 20th element"),
  }
  // &v[100]; // this code will panic because it references a nonexistent element

  // when the program has a valid reference, borrow checker enforces the ownership and borrowing rules to ensure this reference
  // and any other references to the contents of th evector remian valid. Recall the rule that states you can't have mutable
  // and immutable references in the same scope. That rule applies in below; where we hold an immutable reference to the first
  // element in a vector and try to add an element to the end. This program won't work if we also try to refer to that element
  // later
  let mut v = vec![1, 2, 3, 4, 5];

  let first = &v[0];

  v.push(6);

  // println!("The first element is: {}", first);

  //? The code might look like it should work: why should a reference to the first element care about changes at the end of the
  //? vector? This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a
  //? element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if
  //? there isn't enought room to put all the elements next to each other where the vector is currently stored. In that case,
  //? the reference to the first element would be poiting to deallocated memory.

  // Iterating
  let mut v = vec![100, 32, 57];
  for i in &v {
    println!("{}", i);
  }
  println!("---------MAPPING----------");
  // we can also rotate with mutable reference
  // MAP!
  for i in &mut v {
    *i += 50;
  }
  for i in &v {
    println!("{}", i);
  }

  // Want to store different types? Enums are here to rescue!!
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
    Boolean(bool),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
    SpreadsheetCell::Boolean(true),
  ];

  for cell in &row {
    match cell {
      SpreadsheetCell::Int(val) => println!("vector holds an integer value {}", val),
      SpreadsheetCell::Float(val) => println!("vector holds an float value {}", val),
      SpreadsheetCell::Text(val) => println!("vector holds an string value {}", val),
      _ => {} // ingore other types
    }
  }
}
