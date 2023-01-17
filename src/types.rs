// PRIMITIVE TYPES - https://doc.rust-lang.org/book/ch03-02-data-types.html
// Integers: u8, i8, u16, i16, u32, i32, u64, i65, u128, i128, usize, isize
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays

// Rust is statically typed but can infer based on value & usage.

pub fn run() {
    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Default int is i32
    let x = 1;

    // Default float is f64
    let y = 2.5;

    // With explicit type
    let z: i64 = 999999999;

    // Boolean
    let is_active = true;

    // Boolean from expression
    let five_gt_ten = 5 > 10;

    // Char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, five_gt_ten, a1, face));
}
