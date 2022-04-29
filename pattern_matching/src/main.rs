fn main() {
    let v = vec!['a', 'b', 'c'];
    let v_iter = v.iter();

    for (index, value) in v_iter.enumerate() {
        println!("{} is at index {}", value, index);
    }
    println!("{:?}", v);
    // println!("{:?}", v_iter);

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // This new y binding will match any value inside a Some, which is what we have in x.
        // Therefore, this new y binds to the inner value of the Some in x. That value is 5, so the expression
        // for that arm executes and prints Matched, y = 5.
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;

    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    let s = Some(String::from("Hello!"));
    //if let Some(_s) = s {
    // the s value will still be moved into _s, which prevents us from using s again

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // (4 | 5 | 6) if y => println!("yes"),
        _ => println!("no"),
    }

    enum Msg {
        Hello { id: i32 }, // struct
    }

    let msg = Msg::Hello { id: 5 };
    match msg {
        Msg::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Msg::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Msg::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
