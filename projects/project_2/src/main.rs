//Program to determine the years of experience
//of the Employees in a company

use std::io;

fn main() {

println!("Hello dear employee! Please type in your name.");
let mut name = String::new();
io::stdin().read_line(&mut name).expect("Invalid input");

println!("Hello {} welcome! How many years have worked in the company?",name);

//If employee has worked for up to 10 years
//Then they are experienced. If not they are inexperienced

let mut year = String::new();
io::stdin().read_line(&mut year).expect("Invalid input");
let year:i32 = year.trim().parse().expect("Invalid input");

if year >= 10 {
    println!("You are experienced. Very good! Please input your age.");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Invalid input");
    let age:i32 = age.trim().parse().expect("Invalid Input");

    if age >= 40
    {
        println!("{} your incentive is N1,560,000 per month",name);
    }
    else if age >= 30 && age < 40
    {
        println!("{} your incentive is N 1,480,000 per month",name);
    }
    else if age < 28
    {
        println!("{} your incentive is N1,300,000 per month",name);
    }
}
else {
    println!("{} you are inexperienced. Your incentive is N100,000 per month",name);
}
}
