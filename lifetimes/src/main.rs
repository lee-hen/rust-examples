fn main() {
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //         println!("r: {}", r);
    //     }
    //     println!("r: {}", r);
    // }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // The data in novel exists before the ImportantExcerpt
    // instance is created. In addition, novel doesn’t go out of scope until after the
    // ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt
    // instance is valid.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);

    // 'static, which denotes the entire duration of the program.
    let s: &'static str = "I have a static lifetime.";
}

// The lifetime of the reference
// returned by the longest function is the same as the smaller of the lifetimes
// of the references passed in. Therefore, the borrow checker disallows the
// code in Line 28-30 as possibly having an invalid reference.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// the best fix would be to return an owned data type
// rather than a reference so the calling function is then responsible for cleaning
// up the value.
// fn longest_not_compile<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// This
// 'a annotation means an instance of ImportantExcerpt can’t outlive the reference
// it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// The reason this function compiles without lifetime annotations is historical:
// in early versions (pre-1.0) of Rust, this code wouldn’t have compiled
// because every reference needed an explicit lifetime. At that time, the function
// signature would have been written like this:
// fn first_word<'a>(s: &'a str) -> &'a str {
// After writing a lot of Rust code, the Rust team found that Rust programmers
// were entering the same lifetime annotations over and over in particular
// situations. These situations were predictable and followed a few deterministic
// patterns. The developers programmed these patterns into the compiler’s
// code so the borrow checker could infer the lifetimes in these situations and
// wouldn’t need explicit annotations. In the future, even fewer lifetime annotations might be required.

// The patterns programmed into Rust’s analysis of references are called
// the lifetime elision rules.
// Lifetimes on function or method parameters are called input lifetimes,
// and lifetimes on return values are called output lifetimes.
// The compiler uses three rules to figure out what lifetimes references
// have when there aren’t explicit annotations. The first rule applies to input
// lifetimes, and the second and third rules apply to output lifetimes. If the
// compiler gets to the end of the three rules and there are still references
// for which it can’t figure out lifetimes, the compiler will stop with an error.
// These rules apply to fn definitions as well as impl blocks.

// The first rule is that each parameter that is a reference gets its own lifetime
// parameter. In other words, a function with one parameter gets one
// lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters
// gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
// and so on.
// The second rule is if there is exactly one input lifetime parameter, that
// lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32)
// -> &'a i32.
// The third rule is if there are multiple input lifetime parameters, but one
// of them is &self or &mut self because this is a method, the lifetime of self is
// assigned to all output lifetime parameters. This third rule makes methods
// much nicer to read and write because fewer symbols are necessary.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
