/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("5 % 4 = {}", 5 % 4);

    // 32 bit int equal to -4
    let mut neg_4 = -4i32;

    // other functions include .pow .sqrt .cbrt .round .floor .ceil .exp .ln .log10 .to_radians .to_degrees
    // .sin .cos etc...
    println!("Absoulte value: {}", neg_4.abs() );
    println!("Max 4,5: {}", 4f64.max(5f64));
    println!("Min 4,5: {}", 4f64.min(5f64));

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}