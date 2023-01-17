// Iterate until a condition is met

pub fn main() {
    let mut count = 0;

    // Conditional break
    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 20 {
            break;
        }
    }

    println!("----------");
    println!("While loop");
    println!("----------");

    count = 1;

    // While
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    println!("---------");
    println!("For range");
    println!("---------");

    // For range
    for x in 1..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}
