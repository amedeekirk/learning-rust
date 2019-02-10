 /*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    let rand_array = [1,2,3,4,5,];

    println!("{}", rand_array[0]);
    println!("{}", rand_array.len());
    println!("Second Two: {:?}", &rand_array[1..3]);

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}