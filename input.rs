/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    'outer: loop {
        let number: i32 = 10;
        println!("Pick a number");

        loop {
            let mut line = String::new();

            let input = stdin().read_line(&mut line);

            let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

            match guess {
                None => println!("Enter a Number"),
                Some(n) if n == number => {
                    println!("Correct!");
                    break 'outer;
                }
                Some(n) if n < number => println!("Too Low"),
                Some(n) if n > number => println!("Too High"),
                Some(_) => println!("Error"),
            }
        }
    }

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}