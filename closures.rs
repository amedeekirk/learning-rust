/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    
    let sum_nums = |x: i32, y: i32|  x + y;
    println!("7+8={}", sum_nums(7,8));
    
    let num_ten = 10;
    let add_10 = |x: i32| x + num_ten;
    println!("5+10={}", add_10(5));

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}