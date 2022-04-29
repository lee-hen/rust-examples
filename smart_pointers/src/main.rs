#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    // Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum list {
    Cons(Rc<RefCell<i32>>, Rc<list>),
    Nil,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    use crate::List::{Cons, Nil};

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    // println!("{:?}", list);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coercion
    hello(&(*m)[..]);

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    // • From &T to &U when T: Deref<Target=U>
    // • From &mut T to &mut U when T: DerefMut<Target=U>
    // • From &mut T to &U when T: Deref<Target=U>

    // The first two cases are the same except for mutability. The first case
    // states that if you have a &T, and T implements Deref to some type U, you can
    // get a &U transparently. The second case states that the same deref coercion
    // happens for mutable references.
    // The third case is trickier: Rust will also coerce a mutable reference to
    // an immutable one. But the reverse is not possible: immutable references will
    // never coerce to mutable references. Because of the borrowing rules, if you
    // have a mutable reference, that mutable reference must be the only reference
    // to that data (otherwise, the program wouldn’t compile). Converting
    // one mutable reference to one immutable reference will never break the borrowing
    // rules. Converting an immutable reference to a mutable reference
    // would require that there is only one immutable reference to that data, and
    // the borrowing rules don’t guarantee that. Therefore, Rust can’t make the
    // assumption that converting an immutable reference to a mutable reference
    // is possible.

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // c.drop();
    // std::mem::drop(c);
    drop(c);
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // println!("{:?}, {:?}", c, d);

    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // Reference counting, clones that increase the reference count
    // Rc::clone only increments the reference count, which doesn’t take much time.
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // deep-copy, Deep copies of data can take a lot of time.
    let d = a.clone();

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(list::Cons(Rc::clone(&value), Rc::new(list::Nil)));
    let b = list::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = list::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10; // dereference the Rc<T> to the inner RefCell<T>
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
