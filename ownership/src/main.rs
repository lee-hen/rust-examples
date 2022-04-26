fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // this will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        // println!("{}", s);

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to
                   // still use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("{}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("{}", s3);

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{}' is {}.", s5, len);

    let s6 = String::from("hello");
    let len = calculate_len(&s6);
    println!("The length of '{}' is {}.", s6, len);

    let mut s7 = String::from("hello");
    change(&mut s7);
    println!("{}", s7);

    // let mut s8 = String::from("hello");
    // let r1 = &mut s8;
    // let r2 = &mut s8;
    // println!("{}, {}", r1, r2);

    let mut s8 = String::from("hello");
    {
        let r1 = &mut s8;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no
      // problems.
    let r2 = &mut s8;
    println!("{}", r2);

    // let mut s9 = String::from("hello");
    // let r1 = &s9; // no problem
    // let r2 = &s9; // no problem
    // let r3 = &mut s9; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // let reference_to_nothing = dangle();

    let mut s9 = String::from("hello world");
    let word = first_word1(&s9); // word will get the value 5

    println!("{}, {}", s9, word);

    s9.clear(); // this empties the String, making it equal to ""

    println!("{}, {}", s9, word);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally
    // invalid!

    let s10 = String::from("hello world");
    let hello = &s10[0..5];
    let world = &s10[6..11];
    println!("{}", &hello[0..1]);
    println!("{}, {}", hello, world);

    // let mut s11 = String::from("hello world");
    let s11 = String::from("hello world");
    let word = first_word(&s11);
    //  s11.clear(); // error!
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = first_word2(&my_string[..]);
    println!("the first word is: {}", word);

    let word = first_word2(&my_string);
    println!("the first word is: {}", word);

    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word2(&my_string_literal[..]);
    println!("the first word is: {}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word2(my_string_literal);
    println!("the first word is: {}", word);
} // Here, x goes out of scope, then s. But because s's value was moved,
  // nothing special happens.
  // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_len(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of
  // what it refers to, nothing happens.

// fn change(some_string: &String) {
//   some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// // Danger!

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
