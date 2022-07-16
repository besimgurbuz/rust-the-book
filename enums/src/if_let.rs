pub fn main_fn() {
  // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while
  // ignoring the rest. Consider below;
  let config_max = Some(3u8);
  match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
  };

  // Instead, we could write this in a shorter way using if let;
  let config_max = Some(3u8);
  if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
  }
  // in other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern
  // and then ignores all other values.

  // if let also can have an else; (else is same as the _ arm in match)
  let val: Option<i32> = None;

  if let Some(v) = val {
    println!("Value is valid and has {v} value.");
  } else {
    println!("Value is invalid");
  }
}
