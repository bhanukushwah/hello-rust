/*
Primitive Types -
Integers: u8, i8, u16, i16, u32, i32, i64, u64, u128, i128
floats: f32 , f64
Boolean (bool)
Character (char)
Tuples
Arrays
*/

// Rust is statically typed language, but in can infer what types we want to use based on values on complite time

pub fn run() {
    
    // Default Integer
    let a = 1;

    // Floats
    let b: f64 = 292.393;

    // Boolean
    let c: bool = true; // or let c = true

    // Get boolean from expression
    let d:bool = 10<5;

    // Char
    let e = 'a';

    println!("{:?}", (a, b, c, d, e));

}