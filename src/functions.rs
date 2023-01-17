// Store blocks of code for reuse

fn greeting(greet: &str, name: &str) {
    println!("{}, {}! Nice to meet you.", greet, name)
}

fn add(left: i32, right: i32) -> i32 {
    // No semicolon means this is the return statement
    left + right
}

pub fn run() {
    greeting("Hello", "world");

    // Bind function return values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n: i32 = 10;
    let add_nums = |left: i32, right: i32| left + right + n;
    println!("Closure sum: {}", add_nums(3, 3));

    // Note that you won't be able to modify n anymore (even if you declare it
    // as mut) because the closure borrows it. This prevents problems with
    // add_nums returning seeminly strange values when n would've changed
}
