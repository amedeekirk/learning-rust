    /*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    
    say_hello("Amedee");
    println!("5 + 4 = {}", get_sum(5, 4));

    // Binding 
    let sum = get_sum;
    println!("7 + 4 = {}", sum(7, 4));


    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}

fn say_hello(name: &str) {
    println!("Hi {}", name)
}

// -> is the return value type
fn get_sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}