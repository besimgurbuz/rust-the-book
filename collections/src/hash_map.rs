use std::{collections::HashMap, hash::Hash};
// HashMap is not in prelude, so we have to import it with `use`

pub fn hash_map_main() {
    // One way to create an empty has map is using `new` and adding elements with insert.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(val) => println!("Score of {} team is {}", team_name, val),
        None => {}
    }

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    // Hash Maps and Ownership
    // For types that implement the Copy trait, like i32, the values are copied into the has map. For owned values like String,
    // the values will be moved and the hash map will be the owner of those values;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);

    // we aren't able to use the variables `field_name` and `field_value` after they've been moved into the hash map with
    // the call to insert.

    // ! If we insert references to values into the hash map, the values won't moved into the hash map. The values that the
    // ! references point to must be valid for at least as long as the hash map is valid!!. (lifetimes)

    // Updating a Hash Map

    // overwriting
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // ! overwrites

    println!("{:?}", scores);

    // adding a key and value only if a key isn't present
    // Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter. The return
    // value of the `entry` method is an enum called `Entry` that presents a value that might or might not exist. Let's say
    // we want to inser the value 50, and the same for the Blue team. Using the entry API, the code looks like;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // updating the value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    // the `split_whitespace` method returns an iterator over sub-slices, separated by whitespace, of the value in `text`.
    // The `or_insert` method returns a mutable reference (`&mut V`) to the value for the specified key. Here we store that
    // mutable reference in the `count` variable, so in order to assign to that value, we must first dereference `count` using
    // the asterick(*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe
    // and allowed by the borrowing rules

    println!("{:?}", map);
}
