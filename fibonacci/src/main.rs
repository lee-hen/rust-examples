fn main() {
    println!("fibonacci_rec(10): {}", fibonacci_rec(10));
    println!("fibonacci_loop(10): {}", fibonacci_loop(10));

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }
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

struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}
