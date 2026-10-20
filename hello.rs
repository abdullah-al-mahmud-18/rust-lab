fn main() {
    let x = 100;
    println!("x is {}", x);
    println!("Hello Rust!");
    println!("{} days is {} month", 30, 1);

    // Has to start from 0
    println!("{0}, This is {1}. {1}, This is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", object="the lazy dog", verb="jumps over", subject="the quick brown fox");

    println!("Base 10 of {}: {}", x, x);
    println!("Base 2 of {}: {:b}", x, x);
    println!("Base 8 of {}: {:o}", x, x);
    println!("Base 16 of {}: {:x}", x, x);

    // TODO: Try more formatting options
}