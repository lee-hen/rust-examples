static HELLO_WORLD: &str = "Hello, world!";
// static HELLO_WORLD: &'static str = "Hello, world!";

// multiple threads access COUNTER would likely result in data races.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // println!("r1 is: {}", *r1);
    // println!("r2 is: {}", *r2);
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // let address = 0x012345usize;
    // let r = address as *const i32;
    // println!("{:?}", *r);

    // dangerous();

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // use std::slice;
    // let address = 0x012345usize;
    // let r = address as *mut i32;
    // let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }
    // fn returns_long_type() -> Thunk {
    //     // --snip--
    // }

    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
    // a &str is two values: the address of the str and its length
    // we can know the size of a &str value at compile time: it’s twice the length of a usize.

    // That is, we always know the size of a &str, no
    // matter how long the string it refers to is. In general, this is the way in which
    // dynamically sized types are used in Rust: they have an extra bit of metadata
    // that stores the size of the dynamic information. The golden rule of dynamically
    // sized types is that we must always put values of dynamically sized types
    // behind a pointer of some kind.
    // Every trait is a dynamically sized type we can refer to by using the name of the trait.

    // Rust has a particular trait called the Sized trait to
    // determine whether or not a type’s size is known at compile time. This trait
    // is automatically implemented for everything whose size is known at compile
    // time. In addition, Rust implicitly adds a bound on Sized to every generic
    // function. That is, a generic function definition.
    // fn generic<T>(t: T) {
    //     // --snip--
    // }
    // fn generic<T: Sized>(t: T) {
    //     // --snip--
    // }
    // fn generic<T: ?Sized>(t: &T) {
    //     // --snip--
    // }

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // Functions coerce to the type fn (with a lowercase f ), not to
    // be confused with the Fn closure trait. The fn type is called a function pointer.
    // Function pointers implement all three of the closure traits (Fn, FnMut,and FnOnce),
    // so you can always pass a function pointer as an argument for a function that expects a closure.

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // Closures are represented by traits, which means you can’t return closures directly.
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    // Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
    // Attribute-like macros that define custom attributes usable on any item
    // Function-like macros that look like function calls but operate on the tokens specified as their argument
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);
    // (&mut slice[..mid], &mut slice[mid..])

    use std::slice;
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// a trait with one method and an associated type.
// trait Add<RHS = Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }
// RHS=Self: this syntax is called default type parameters.
// The RHS generic type parameter defines the type of the rhs parameter in the add method. If we don’t specify
// a concrete type for RHS when we implement the Add trait, the type of RHS will
// default to Self, which will be the type we’re implementing Add on.

impl Add for Point {
    type Output = Point; // associated type
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
