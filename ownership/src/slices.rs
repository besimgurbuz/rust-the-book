pub fn main_fn() {
  // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is
  // a kind of reference, so it does not have ownership

  // Example: Write a function that takes a string of words separated by spaces and returns the first word it finds in that
  // string. If the function doesn't find a space in the string, the whole string must be one word, so the entire string
  // should be returned.
  // How would be the signature of this function?
  // fn first_word(s: &String) -> ?
  // The `first_word` function has a &String as a parameter. We don't want ownership, so this is fine. But what should we
  // return? We don't really have a way to talk about part of a string. However, we could return the index of the end of the
  // word, indicated by a space. Let's try;

  // There's a problem. We're returning a `usize` on its own, but it's only a meningful number in the context of the &String.
  // In other words, because it's a seprate value from the String, there's no guarantee that it will still be valid in the
  // future. Consider below;
  let mut s = String::from("hello world");
  let word = first_word_without_slice(&s); // word will get the value 5

  s.clear(); // this empties the String, making it equal to ""
             // word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with. Word is now
             // totally invalid.

  // If we used word after calling s.clear(). Because word isn't connected to the state of s it will still contain the value 5
  // but if we try to extract the first word out with the value 5, this would be a bug because content of s have changed

  // String Slices
  // A string slice is a reference to part of a String, and it looks like this:
  let s = String::from("hello world");
  let hello = &s[0..5];
  let world = &s[6..11];

  // rather than a reference to the entire String, hello is a refernece to a portion of the String specified in the extra
  // [0..5] bit. We create slices using a range withing brackets by specifying [starting_index..ending_index], where
  // starting_index is the first position in the slice and ending_index is one more than the last position in the slice.
  // Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to
  // ending_index minus starting_index. So in the case of let world = &s[6..11];, world would be slice that contains a pointer
  // to the byte at index 6 of s with a length of value 5.

  // if you want to start from 0 index you can drop the value before two periods
  let slice = &s[0..2];
  let slice = &s[..2];
  // are actually equal
  // and if your slice includes the last byte of the String, you can drop the trailling number. That means these are equal:
  let len = s.len();
  let slice = &s[3..len];
  let slice = &s[3..];
  // you can also drop both values to take a slice of the entire string, so these are equal
  let slice = &s[0..len];
  let slice = &s[..];

  // let's rewrite the first_word to return a slice. The type that signifies "string slice" is written as &str:
  let first = first_word(&s);
  let second = second_word(&s);
  println!("first word of '{s}' is {first}");
  println!("second word of '{s}' is {second}");

  // we now have a straightforward API that's much harder to mess up, because the compiler will ensure the references into
  // the String remain valid. Remember the bug when we got the index to the end of the first word but then cleared the string
  // so our index was invalid? That code was logically incorrect but didn't show any immediate errors. The problems would show
  // up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us
  // know we have a problem with our code much sooner.

  let mut s = String::from("hello world");
  let word = first_word(&s);
  // s.clear(); // error!

  println!("the first word of '{}' is: {}", s, word);
  // Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable
  // reference. Because clear needs to truncate the String, it needs to get a mutable reference. The println! after the call
  // to clear uses the reference in word, so the immutable reference must still be active at that point. Rust disallows the
  // mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails.

  // String literals are slices
  let s = "Hello, world!";
  // The type oof s here is &str: it's a slice pointing to that specific point of the binary. This is also why string literals
  // are immutable; &str is an immutable reference.

  // String slices as Parameters
  // Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that's
  // it's signature:
  // fn first_word(s: &str) -> &str {}
  // If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a
  // reference to the String. This flexibility takes advantage of deref coercions;
  let my_string = String::from("hello world");

  // `first_word_final` works on slices oof `String`s, wheter partial or whole
  let word = first_word_final(&my_string[0..6]);
  let word = first_word_final(&my_string[..]);
  // `first_word_fina` also works on references to `String`s, which are equivalent to whole slices of `String`s
  let word = first_word_final(&my_string);
  let my_string_literal = "hello world";

  // `first_word_final` works on slices of string literals, whether partial or whole
  let word = first_word_final(&my_string_literal[0..6]);
  let word = first_word_final(&my_string_literal[..]);
  // because string literals *are* string slices already, this works too, without the slice syntax
  let word = first_word_final(my_string_literal);

  // Other slices
  // String slices, as you might imagine, are specific to strings. But there's a more general slice type, too. Consider this array
  let a = [1, 2, 3, 4, 5];

  let slice = &a[1..3];
  assert_eq!(slice, &[2, 3]);
}

fn first_word_final(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  s
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

fn second_word(s: &String) -> &str {
  let bytes = s.as_bytes();
  let mut first_word_ended_index: usize = 0;

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      if first_word_ended_index != 0 {
        return &s[first_word_ended_index..i];
      } else {
        first_word_ended_index = i + 1;
      }
    }
  }
  if first_word_ended_index != 0 {
    return &s[first_word_ended_index..];
  }
  ""
}

fn first_word_without_slice(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  return s.len();
}
