/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    // Basic conditionals != == > < >= <=
    // Basic operators && || !

    let age_old = 6;

    if(age_old == 5) { 
        println!("Go to Kindergarten");
    }
    else if(age_old > 5 && age_old <= 18) {
        println!("Go to grade {}", (age_old - 5));
    }
    else if (age_old <= 25 && age_old > 18) {
        println!("Go to college");
    }
    else {
        println!("Go to work");
    }

    // The closest thing Rust has to a ternary at this point
    let can_vote = if (age_old >= 18) {true} else {false};

    println!("Can Vote: {}", can_vote);

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}