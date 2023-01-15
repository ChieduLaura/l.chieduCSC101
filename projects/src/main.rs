// Rust program to determine 
// the roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first value: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a value number!");

    println!("Enter second value: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number!");

    println!("Enter third value: ");
    io::stdin().read_line(&mut input3).expect("Not a value string");
    let c:f32 = input3.trim().parse().expect("Not a valid number!");

    let discriminant:f32 = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        println!("There are two distinct roots.");
    }
    else if discriminant == 0.0 {
        println!("There is only one real root.");
    }
    else if discriminant < 0.0 {
        println!("There are no real roots.");
    }
}