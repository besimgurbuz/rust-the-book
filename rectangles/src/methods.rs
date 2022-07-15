#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // `&self` is short for self: &Self
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn increase(&mut self) {
    self.width = self.width + 1;
    self.height = self.height + 1;
  }
  // we can also use field names for method names, Rust can understand the difference (width, width())
  fn width(&self) -> bool {
    self.width > 0
  }
  fn can_hold(&self, other_rect: &Rectangle) -> bool {
    self.width > other_rect.width && self.height > other_rect.height
  }
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

pub fn main_fn() {
  let mut rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  println!(
    "methods: The area of the rectangle is {} square pixels",
    rect1.area()
  );

  rect1.increase();
  println!(
    "methods: The area of the rectangle is {} square pixels",
    rect1.area()
  );

  if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}", rect1.width);
  }

  // Rust has a feature called automatic referencing and dereferencing,
  // Calling methods is one of the few places in Rust that has this behavior
  // Here's how it works: when you call a method with object.something(), Rust
  // automaticlly adds in `&`, `&mut` or `*` so object matches the signature
  // of the method. In other words, the following are the same;
  // p1.distance(&p2);
  // (&p1).distance(&p2);

  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle::square(32);

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  // Associated Functions
  // All functions defined within an impl block are called associated functions
  // because they're associated with the type named after the imp. We can define
  // associated functions that don't have self as their first parameter (and
  // thus are not methdos) because they don't need an instance of the type to
  // work with. We've already used one function like this: the `String::from`
  // function that's defined on the String type
  // Associated functions that aren't methods are often used for constructions that will return a new instance of the struct
  // For example, we could provide an associated function that woould have one dimension parameter and use that as both
  // width and height, thus making it easier to create a square Rectangle rahter than havinng to specify the same value twice:
  // impl Rectangle {
  //   fn square(size: u32) -> Rectangle {
  //     Rectangle {
  //       width: size,
  //       height: size,
  //     }
  //   }
  // }
  // to call this associated function, we use the `::` syntax with the struct name; `let sq = Rectangle::square(3);`
  // this function is namespaced by the struct: the `::` syntax is used for both associated functions and namespaces created
  // by modules.

  // Multiple impl Blocks
  // Each struct is allowed to have multiple impl blocks. For example,
  //   impl Rectangle {
  //     fn area(&self) -> u32 {
  //         self.width * self.height
  //     }
  // }

  // impl Rectangle {
  //     fn can_hold(&self, other: &Rectangle) -> bool {
  //         self.width > other.width && self.height > other.height
  //     }
  // }
  // is equal to;
  // impl Rectangle {
  //   fn area(&self) -> u32 {
  //     self.width * self.height
  //   }
  //   fn can_hold(&self, other: &Rectangle) -> bool {
  //     self.width > other.width && self.height > other.height
  //   }
  // }
}
