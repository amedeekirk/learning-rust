/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    // Fixed sized lists w/ keys
    let rand_tuble = ("Amedee", 22);

    let rt2: (&str, i8) = ("Amedee", 22);

    println!("Name: {}", rt2.0);

    
    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}