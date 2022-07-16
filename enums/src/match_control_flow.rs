enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1, // if match arm code is short, there is no need to use curly brackets
    Coin::Nickel => {
      println!("Nickel!");
      5
    }
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

pub fn main_fn() {
  let quarter_in_cents = value_in_cents(Coin::Quarter);

  println!("1 quarter equals to {quarter_in_cents} cents.");

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  // Catch-all Patterns and the _ placeholder
  let dice_roll = 9;
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // will match all values not specifically listed. (catch_all)
                                 // if we add any arm after a catch-all, Rust will warn us, because they will never match
  }
  // Rust also has a pattern we can use when we don't wan ttoo use the value in the catch-all pattern: `_`,
  // which is a special pattern that matches any value and does not bind to that value. This tells Rust aren't going to use
  // the value
  match dice_roll {
    3 => (), // nothing happens
    _ => reroll(),
  }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
