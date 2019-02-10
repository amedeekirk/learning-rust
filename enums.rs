/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
    let spiderman = Hero::Info {name: "Spiderman".to_owned(), secret: "Peter Parker".to_owned()};

    get_info(hulk);
    get_info(spiderman);

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}

enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}

fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info {name, secret} => {
            println!("{} is {}", name, secret);
        }
    }
}