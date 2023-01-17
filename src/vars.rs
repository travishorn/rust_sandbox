// Variables hold primitive data or references
// Immutable and block-scoped

pub fn main() {
    let name = "Mike";
    let mut age = 30;

    println!("My name is {} and I am {} years old.", name, age);

    age = 31;

    println!("My name is {} and I am {} years old.", name, age);

    // Constant - type required. Usually all caps.
    const ID: i32 = 1;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Mike", 31);
    println!("{} is {} years old.", my_name, my_age);
}
