/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    let rand_str = "Random string";

    println!("Length: {}", rand_str.len());

    let (first, second) = rand_str.split_at(6);
    println!("First: {} Second: {}", first, second );

    let count = rand_str.chars().count();
    let mut chars = rand_str.chars();

    let mut ind_char = chars.next();

    loop {
        match ind_char {
            Some(x) => println!("{}", x),
            None => break,
        }

        ind_char = chars.next();
    }

    let mut iter = rand_str.split_whitespace();

    let mut ind_word = iter.next();

    loop {
        match ind_word {
            Some(x) => println!("{}", x),
            None => break,
        }
        ind_word = iter.next();
    }

    let rand_str2 = "hi\nnew\nlines\n! ";
    let mut lines = rand_str2.lines();
    let mut ind_line = lines.next();

    loop {
        match ind_line {
            Some(x) => println!("{}", x),
            None => break,
        }
        ind_line = lines.next();
    }

    println!("Find new: {}", rand_str2.contains("new"));

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");
}
