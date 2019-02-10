/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/

//  i - integers of 8-64 bits. u - unsigned int. i/usize depends on system. f - floats 
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {

    /*
    *           Variables
    */

    // Rust can infer type based on assigned value
    // Once assigned, it is immutable by default
    let num = 10;

    // make variable mutable
    let mut age: i32 = 21;
    age = 22;

    // Some basic variable type sizes
    println!("Max isize {}", isize::MAX);
    println!("Min isize {}", isize::MIN);
    println!("Max usize {}", usize::MAX);
    println!("Min isize {}", usize::MIN);
    println!("Max i8 {}", i8::MAX);
    println!("Min i8 {}", i8::MIN);

    // Var names must start with letter, can contain underscores
    let (f_name, l_name) = ("Amedee", "Kirkpatrick");

    println!("My name is {} {}", f_name, l_name);
    println!("I'm {} years old", age );
    
    /*
    *           6:41
    */

    // Other variable types include bool and string
    let is_it_true: bool = true; 
    let let_x: char = 'x';

    println!("It's {0} that {1} is {0}", is_it_true, let_x );
    
    println!("Truncate to 2 decimals: {:.2}", 1.234);
    println!("Binary: {:b}, Hexadecimal: {:x}, Octal: {:o}", 10, 10, 10);

    // Named arguments + whitespace such that the total characters will be at lest 5
    println!("{ten:>ws$}", ten=10, ws=5);

    //Use 0 instead of whitespace
    println!("{ten:>0ws$}", ten=10, ws=5);

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");   

}