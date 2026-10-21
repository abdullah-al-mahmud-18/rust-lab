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

    // Pad white spaces to the left of the number. 
    // Total 5 characters will be printed, 4 white spaces and 1 character for the number. 
    println!("{number:>5}", number=5);

    // Pad the character 0 to the left of the number. 
    println!("{number:0>5}", number=5);

    // Same as above, but the width is specified by a variable.
    println!("{number:0>width$}", number=5, width=5);
}