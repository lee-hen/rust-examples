// fn add_one_v1 (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x| { x + 1 };
// let add_one_v4 = |x| x + 1 ;

fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("{}", s);
    // let n = example_closure(5);
    // println("{}", s);

    let x = 4;
    let equal_to_x = |z| z == x;
    // fn equal_to_x(z: i32) -> bool { z == x }
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    // println!("{:?}", v1_iter);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    assert_eq!(v1, vec![1, 2, 3]);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    // produces an iterator over immutable references
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    println!("{:?}", v1_iter);
    println!("{:?}", v1);
}

#[test]
fn iterator_demonstration1() {
    let v1 = vec![1, 2, 3];
    // takes ownership of v1 and returns owned values
    let mut v1_iter = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
    println!("{:?}", v1_iter);
    // println!("{:?}", v1);
}

#[test]
fn iterator_demonstration2() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();
    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
