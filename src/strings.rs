// Primitive str is immutable and fixed-length in memory.
// String is growable and heap-allocated. Use when you need to modify data.

pub fn run() {
    // Won't work because hello is a prim string (immutable and fixed-length)
    // let mut hello = "Hello, ";

    // hello is now a String (growable and heap-allocated)
    let mut hello = String::from("Hello, ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('w');

    // Push string
    hello.push_str("orld!");

    println!("{}", hello);

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Is empty
    println!("Is empty? {}", hello.is_empty());

    // Contains a substring?
    println!("Contains \"world\"? {}", hello.contains("world"));

    // Replace
    println!("Replaced: {}", hello.replace("world", "universe"));

    // Loop through string by whitespace
    for token in hello.split_whitespace() {
        println!("{}", token);
    }

    // There are more string methods. Those are just a sample.

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertions
    assert_eq!(2, s.len()); // true, no visual result
    assert_eq!(11, s.capacity()); // false, thread panic, error
}
