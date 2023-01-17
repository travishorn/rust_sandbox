// Grouped values of different types. Max 12 elements

pub fn main() {
    let person: (&str, &str, i8) = ("Mike", "New Jersey", 31);
    println!(
        "{} is from {} and is {} years old.",
        person.0, person.1, person.2
    );
}
