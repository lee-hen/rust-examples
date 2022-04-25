fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   　";
    let spaces = spaces.len();

    // let mut spaces = " ";
    // spaces = spaces.len();
    println!("{}", spaces);

    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    let c = 'z';
    println!("{}", c);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{}", months[0]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    let a = [3; 5];
    println!("{}", a[4]);
}
