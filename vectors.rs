/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    let mut vect = vec![1,2,3,4,5];
    println!("Item 2: {}", vect[1]);

    for i in &vect {
        println!("Vect: {}", i);
    }

    // Add and remove values on array
    vect.push(6);
    vect.pop();

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}