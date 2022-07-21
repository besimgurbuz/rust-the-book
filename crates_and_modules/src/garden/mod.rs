mod fruits;
pub mod vegetables;

pub fn hello() {
  println!("hello from garden!");

  fruits::hello();
  vegetables::hello();
}
