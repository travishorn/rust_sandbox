// Used to act based on condition of something

pub fn run() {
    let mut age: u8 = 18;
    let mut checked_id: bool = false;
    let knows_person: bool = true;

    // if with true condition
    if age >= 21 {
        println!("Bartender: What would you like to drink?");
    } else {
        println!("Bartender: Sorry, you have to leave.");
    }

    // if with false condition
    age = 22;

    if age >= 21 {
        println!("Bartender: What would you like to drink?");
    } else {
        println!("Bartender: Sorry, you have to leave.");
    }

    // && (and)
    if age >= 21 && checked_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && checked_id {
        println!("Bartender: Sorry, you have to leave.");
    } else {
        println!("Bartender: I'll need to see your ID.");
    }

    checked_id = true;

    if age >= 21 && checked_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && checked_id {
        println!("Bartender: Sorry, you have to leave.");
    } else {
        println!("Bartender: I'll need to see your ID.");
    }

    // || (or)
    if age >= 21 && (checked_id || knows_person) {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && (checked_id || knows_person) {
        println!("Bartender: Sorry, you have to leave.");
    } else {
        println!("Bartender: I'll need to see your ID.");
    }

    // if one-liner (no ternary operator in Rust)
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
