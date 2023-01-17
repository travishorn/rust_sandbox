// Very important in Rust. Similar to classes. Contains members, attributes, and
// functions related to the struct. They are used to create custom data types.

// Traditional struct
// First letter uppercase
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct TColor(u8, u8, u8);

struct Person {
    name: String,
    age: u8,
}

// Can write associated functions with impl
impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age: age,
        }
    }

    // Self is similar to "this" in other languages
    fn intro(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }

    // Set age. Since we're chaning something, use &mut
    fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    // Attributes to tuple
    fn to_tuple(self) -> (String, u8) {
        // Remember no semicolon because we're returning
        (self.name, self.age)
    }
}

pub fn main() {
    // Assign values with the property names
    let mut french_rose = Color {
        red: 200,
        green: 71,
        blue: 111,
    };

    // Since its mut, you can reassign values
    french_rose.red = 239;

    // Access by name
    println!(
        "Color: {} {} {}",
        french_rose.red, french_rose.green, french_rose.blue
    );

    // Assign values by their position
    let mut caribbean_green = TColor(0, 214, 160);

    // Reassign and access via index
    caribbean_green.0 = 6;

    println!(
        "Color: {} {} {}",
        caribbean_green.0, caribbean_green.1, caribbean_green.2
    );

    let mut mike = Person::new("Mike", 30);

    println!("{} is {} years old.", mike.name, mike.age);

    // Uses the intro function from impl above. Should return identical string
    // to the line above
    println!("{}", mike.intro());

    mike.set_age(31);
    println!("{}", mike.intro());

    // Use to_tuple from impl above
    println!("{:?}", mike.to_tuple());
}
