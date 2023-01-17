// Types that have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending on variant
    // match is like switch in other languages
    match m {
        Movement::Up => println!("Avatar moving up."),
        Movement::Right => println!("Avatar moving right."),
        Movement::Down => println!("Avatar moving down."),
        Movement::Left => println!("Avatar moving left."),
    }
}

pub fn main() {
    move_avatar(Movement::Up);
    move_avatar(Movement::Right);
    move_avatar(Movement::Down);
    move_avatar(Movement::Left);
}
