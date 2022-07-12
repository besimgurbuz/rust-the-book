pub fn generate_nth_fibonacci_number(nth: i32) -> i32 {
  let mut x = 0;
  let mut y = 1;

  for _ in 1..nth {
    let temp = y;
    y = x + y;
    x = temp;
  }

  return x;
}
