/*
*           Follow Along code for learning rust with Derek Banas
*           https://www.youtube.com/watch?v=U1EFgCNLDB8
*/
use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {

    let mut c1 = Circle {
        x: 10.0, y:10.0, radius: 10.0
    };

    println!("X: {} Y: {} R: {}", c1.x, c1.y, c1.radius);
    println!("Radius: {}", get_radius(&c1));

    println!("Circle x: {}", c1.get_x());

    println!("Circle area: {}", c1.area());

    let mut r1 = Rect {
        height: 10.0, width: 10.0
    };


    println!("Circle area: {}", r1.area());

    // Keeps terminal open
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
}

struct Circle  { 
    x: f64,
    y: f64,
    radius: f64
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

struct Rect {
    height: f64, 
    width: f64
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14 * (self.radius * self.radius)
    }
}

impl HasArea for Rect {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}
