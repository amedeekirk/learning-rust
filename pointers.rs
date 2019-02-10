/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    let v1 = vec![1,2,3];
    let v2 = v1;

    // If you assign data to another variable to another variable for non-primitives, the original can no longer access it
    // Hence, the following line does not work:
    // println!("v1[0]={}", v1[0]);

    // Works fine for primitives, however
    let prim1 = 1;
    let prim2 = prim1;
    println!("{}", prim1);

    let v2 = vec![1,2,3,4];
    println!("Sum of vector: {}", sum_vect(&v2));
    println!("Vector: {:?}", v2);

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}

// If we use a reference, however (&), we can use a copy of a non-primitive safely
fn sum_vect(v1: &Vec<i32>) -> i32 {
    // Fold is iterator adapter that applies a function to all the values. Takes an initial value and closure
    let sum = v1.iter().fold(0, |mut sum, &x| {sum+=x; sum});
    return sum;
}