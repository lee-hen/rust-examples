fn main() {
    println!("fibonacci_rec(10): {}", fibonacci_rec(10));
    println!("fibonacci_loop(10): {}", fibonacci_loop(10));
}

fn fibonacci_rec(i: u32) -> u32 {
    if i == 0 {
        return 0;
    } else if i == 1 {
        return 1;
    } else {
        fibonacci_rec(i - 1) + fibonacci_rec(i - 2)
    }
}

fn fibonacci_loop(i: u32) -> u32 {
    if i == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;

    let mut index = 2;
    while index < i {
        let c = a + b;
        a = b;
        b = c;

        index = index + 1;
    }

    a + b
}
